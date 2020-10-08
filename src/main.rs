
use chrono::prelude::*;
use db::DB;

use serde::{Deserialize, Serialize};
use std::convert::Infallible;
use warp::{Filter, Rejection};


type Result<T> = std::result::Result<T, error::Error>;
type WebResult<T> = std::result::Result<T, Rejection>;
#[macro_use]
mod db;
mod error;
mod handlers;
mod schemas;

#[tokio::main]
async fn main() -> Result<()> {
    let db = DB::init().await?;

    let candidate = warp::path("candidate");

    let candidate_routes = candidate
        .and(warp::post())
        .and(warp::body::json())
        .and(with_db(db.clone()))
        .and_then(handlers::create_candidate_handler)
        .or(candidate
            .and(warp::put())
            .and(warp::path::param())
            .and(warp::body::json())
            .and(with_db(db.clone()))
            .and_then(handlers::edit_candidate_handler))
        .or(candidate
            .and(warp::delete())
            .and(warp::path::param())
            .and(with_db(db.clone()))
            .and_then(handlers::delete_candidate_handler))
        .or(candidate
            .and(warp::get())
            .and(with_db(db.clone()))
            .and_then(handlers::candidate_list_handler));

    let routes = candidate_routes.recover(error::handle_rejection);

    println!("Started on port 4000");
    warp::serve(routes).run(([127, 0, 0, 1], 4000)).await;
    Ok(())
}

fn with_db(db: DB) -> impl Filter<Extract = (DB,), Error = Infallible> + Clone {
    warp::any().map(move || db.clone())
}
