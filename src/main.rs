use std::{
    net::{IpAddr, SocketAddr},
    path::PathBuf,
};

use axum::{extract::Query, http::StatusCode, routing::get, Json, Router};
use clap::Parser;
use ip2proxy::{Columns, Database, Row};
use serde::{Deserialize, Serialize};

#[derive(Parser)]
struct Opt {
    /// Listen on this socket address.
    #[clap(long, default_value = "127.0.0.1:1929")]
    bind: SocketAddr,
    /// Database file to serve.
    #[clap(parse(from_os_str))]
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

#[derive(Deserialize)]
struct BatchQuery {
    #[serde(with = "serde_with::rust::StringWithSeparator::<serde_with::CommaSeparator>")]
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
        Box::leak(Box::new(Database::open(opt.db).expect("open bin database")));

    let app = Router::new()
        .route("/", get(move |query| simple_query(db, query)))
        .route("/batch", get(move |query| batch_query(db, query)))
        .route("/status", get(move || status(db)));

    axum::Server::bind(&opt.bind)
        .serve(app.into_make_service())
        .await
        .expect("bind");
}
