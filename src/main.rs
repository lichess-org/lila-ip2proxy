use std::net::{IpAddr, SocketAddr};
use std::path::PathBuf;
use clap::Clap;
use warp::Filter;
use serde::{Serialize, Deserialize};
use ip2proxy::{Columns, Database};

#[derive(Clap)]
struct Opt {
    /// Listen on this address
    #[clap(long = "address", default_value = "127.0.0.1")]
    address: String,
    /// Listen on this port
    #[clap(long = "port", default_value = "1929")]
    port: u16,
    /// Database file to serve
    #[clap(parse(from_os_str))]
    db: PathBuf,
}

#[derive(Deserialize)]
struct Query {
    ip: IpAddr,
}

#[derive(Debug)]
struct DatabaseError(std::io::Error);

impl warp::reject::Reject for DatabaseError { }

async fn query(db: &'static Database, query: Query) -> Result<impl warp::Reply, warp::Rejection> {
    match db.query(query.ip, Columns::all()) {
        Ok(Some(row)) => Ok(warp::reply::json(&row)),
        Ok(None) => Err(warp::reject::not_found()),
        Err(err) => Err(warp::reject::custom(DatabaseError(err))),
    }
}

#[derive(Deserialize)]
struct BatchQuery {
    #[serde(with = "serde_with::rust::StringWithSeparator::<serde_with::CommaSeparator>")]
    ips: Vec<IpAddr>,
}

async fn batch_query(db: &'static Database, query: BatchQuery) -> Result<impl warp::Reply, warp::Rejection> {
    let mut response = Vec::with_capacity(query.ips.len());
    for ip in query.ips {
        response.push(match db.query(ip, Columns::all()) {
            Ok(row) => row,
            Err(err) => return Err(warp::reject::custom(DatabaseError(err))),
        });
    }
    Ok(warp::reply::json(&response))
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

fn status(db: &'static Database) -> impl::warp::Reply {
    warp::reply::json(&Status {
        px: db.header().px(),
        day: db.header().day(),
        month: db.header().month(),
        year: db.header().year(),
        rows_ipv4: db.header().rows_ipv4(),
        rows_ipv6: db.header().rows_ipv6(),
    })
}

#[tokio::main]
async fn main() {
    let opt = Opt::parse();
    let bind = SocketAddr::new(opt.address.parse().expect("valid address"), opt.port);

    let db: &'static Database = Box::leak(Box::new(Database::open(opt.db).expect("valid bin database")));

    let simple = warp::path::end()
        .and(warp::get())
        .map(move || db)
        .and(warp::query::query())
        .and_then(query);

    let batch = warp::path!("batch")
        .and(warp::get())
        .map(move || db)
        .and(warp::query::query())
        .and_then(batch_query);

    let status = warp::path!("status")
        .and(warp::get())
        .map(move || db)
        .map(status);

    warp::serve(simple.or(batch).or(status)).run(bind).await;
}
