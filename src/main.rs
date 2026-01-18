use std::{
    net::{IpAddr, SocketAddr},
    path::PathBuf,
};

use axum::{extract::Query, http::StatusCode, routing::get, Json, Router};
use clap::Parser;
use ip2proxy::{Columns, Database, Row};
use listenfd::ListenFd;
use serde::{Deserialize, Serialize};
use serde_with::{formats::CommaSeparator, serde_as, StringWithSeparator};
use tikv_jemallocator::Jemalloc;
use tokio::net::{TcpListener, UnixListener};

#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

#[derive(Parser, Debug)]
struct Opt {
    /// Listen on this socket address.
    #[arg(long, default_value = "127.0.0.1:1929", env = "LILA_IP2PROXY_BIND")]
    bind: SocketAddr,

    /// Path to the IP2Proxy BIN database file.
    #[arg(long, env = "LILA_IP2PROXY_DB")]
    db: PathBuf,
}

#[derive(Deserialize)]
struct SimpleQuery {
    ip: IpAddr,
}

async fn simple_query(
    db: &'static Database,
    Query(query): Query<SimpleQuery>,
) -> Result<Json<Row>, StatusCode> {
    db.query(query.ip, Columns::all())
        .expect("simple query")
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

#[serde_as]
#[derive(Deserialize)]
struct BatchQuery {
    #[serde_as(as = "StringWithSeparator::<CommaSeparator, IpAddr>")]
    ips: Vec<IpAddr>,
}

async fn batch_query(
    db: &'static Database,
    Query(query): Query<BatchQuery>,
) -> Json<Vec<Option<Row>>> {
    Json(
        query
            .ips
            .into_iter()
            .map(|ip| db.query(ip, Columns::all()).expect("batch query"))
            .collect(),
    )
}

#[derive(Serialize)]
struct Status {
    px: u8,
    day: u8,
    month: u8,
    year: u8,
    rows_ipv4: u32,
    rows_ipv6: u32,
}

async fn status(db: &'static Database) -> Json<Status> {
    Json(Status {
        px: db.package_version(),
        day: db.day(),
        month: db.month(),
        year: db.year(),
        rows_ipv4: db.rows_ipv4(),
        rows_ipv6: db.rows_ipv6(),
    })
}

#[tokio::main]
async fn main() {
    let opt = Opt::parse();

    let db: &'static Database =
        Box::leak(Box::new(Database::open(&opt.db).expect("open bin database")));

    let app = Router::new()
        .route("/", get(move |query| simple_query(db, query)))
        .route("/batch", get(move |query| batch_query(db, query)))
        .route("/status", get(move || status(db)));

    let mut fds = ListenFd::from_env();
    if let Ok(Some(uds)) = fds.take_unix_listener(0) {
        uds.set_nonblocking(true).expect("set nonblocking");
        let listener = UnixListener::from_std(uds).expect("listener");
        axum::serve(listener, app).await.expect("serve");
    } else if let Ok(Some(tcp)) = fds.take_tcp_listener(0) {
        tcp.set_nonblocking(true).expect("set nonblocking");
        let listener = TcpListener::from_std(tcp).expect("listener");
        axum::serve(listener, app).await.expect("serve");
    } else {
        let listener = TcpListener::bind(&opt.bind).await.expect("bind");
        axum::serve(listener, app).await.expect("serve");
    }
}
