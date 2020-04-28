use std::net::{IpAddr, SocketAddr};
use std::path::PathBuf;
use structopt::StructOpt;
use warp::Filter;
use serde::Deserialize;
use ip2proxy::{Columns, Database};

#[derive(StructOpt)]
struct Opt {
    /// Listen on this address
    #[structopt(long = "address", default_value = "127.0.0.1")]
    address: String,
    /// Listen on this port
    #[structopt(long = "port", default_value = "1929")]
    port: u16,
    /// Database file to serve
    #[structopt(parse(from_os_str))]
    file: PathBuf,
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

#[tokio::main]
async fn main() {
    let opt = Opt::from_args();
    let bind = SocketAddr::new(opt.address.parse().expect("valid address"), opt.port);

    let db: &'static Database = Box::leak(Box::new(Database::open(opt.file).expect("valid bin database")));

    let route = warp::get()
        .map(move || db)
        .and(warp::query::query())
        .and_then(query);

    warp::serve(route).run(bind).await;
}
