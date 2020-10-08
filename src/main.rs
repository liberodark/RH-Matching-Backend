use chrono::prelude::*;
use db::DB;
use serde::{Deserialize, Serialize};
use std::convert::Infallible;
use warp::{Filter, Rejection};

type Result<T> = std::result::Result<T, error::Error>;
type WebResult<T> = std::result::Result<T, Rejection>;


mod db;
mod error;
mod handler;



#[derive(Serialize, Deserialize, Debug)]
pub struct Candidate {
    pub id: String,
    pub firstName: String,
    pub lastName: String,
    pub statusCandidate: String,
    pub statusIndex: String,
    pub statusDate: String,
    pub Email: String,
    pub phoneNumber: String,
    pub postTitle: String,
    pub origin: String,
    pub customer: String,
    pub experience: String,
    pub salary: String,
    pub availabilityDate: String,
    pub mangerName: Vec<String>,
    pub CrName: String,
    pub KoTag: String,
    pub cvCandidat: String,
    pub needReference: Vec<String>,
    pub needReferenceId: Vec<String>,
    pub comment: String,
    pub mobility: String,
    pub tags: Vec<String>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Need {
    id: String,
    postName: String,
    location:String,
    customer: String,
    experience: String,
    maxSalary: String,
    StartDate: String,
    creationDate: String,
    managerName: String,
    CrName: String,
    referenceNeed: String,
    statusNeed: String,
    statusIndex:String,
    affectedCandidatList: Vec<String>,
    tags: Vec<String>,
}
#[tokio::main]
async fn main() -> Result<()> {
    let db = DB::init().await?;

   // let candidate_path = warp::path("api/V1");

    let candidate_routes = 
        (warp::path!("api" / "candidat"))
        .and(warp::post())
        .and(warp::body::json())
        .and(with_db(db.clone()))
        .and_then(handler::create_candidate_handler)
        .or((warp::path!("api" / "candidat"))
            .and(warp::put())
            .and(warp::path::param())
            .and(warp::body::json())
            .and(with_db(db.clone()))
            .and_then(handler::edit_candidate_handler))
        .or((warp::path!("api" / "candidat"))
            .and(warp::delete())
            .and(warp::path::param())
            .and(with_db(db.clone()))
            .and_then(handler::delete_candidate_handler))
        .or((warp::path!("api" / "candidat"))
            .and(warp::get())
            .and(with_db(db.clone()))
            .and_then(handler::candidate_list_handler));

    let routes = candidate_routes.recover(error::handle_rejection);

    println!("Started on port 4000");
    warp::serve(routes).run(([127, 0, 0, 1], 4000)).await;
    Ok(())
}

fn with_db(db: DB) -> impl Filter<Extract = (DB,), Error = Infallible> + Clone {
    warp::any().map(move || db.clone())
}
