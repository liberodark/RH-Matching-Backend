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
     id: String,
     firstName: String,
     lastName: String,
     statusCandidate: String,
     statusIndex: String,
     statusDate: String,
     Email: String,
     phoneNumber: String,
     postTitle: String,
     origin: String,
     customer: String,
     experience: String,
     salary: String,
     availabilityDate: String,
     mangerName: Vec<String>,
     CrName: String,
     KoTag: String,
     cvCandidat: String,
     needReference: Vec<String>,
     needReferenceId: Vec<String>,
     comment: String,
     mobility: String,
     tags: Vec<String>,
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

//config Structure 
// pub struct Follow_Config{
//     need_Status:  Vec<String>,
//     candidate_Status: Vec<String>,
//     Cr_Names: Vec<String>,
//     Manager_Name: Vec<String>,
//     customer: Vec<String>, 
//     mobility: Vec<String>,
//     experience: Vec<String>,
//     KoTag: Vec<String>,
//     origin: Vec<String>,
// }
#[tokio::main]
async fn main() -> Result<()> {
    let db = DB::init().await?;

   // let candidate_path = warp::path("api/V1");

    let candidate_routes = 
        ((warp::path!("api" / "candidat"))
        .and(warp::post())
        .and(warp::body::json())
        .and(with_db(db.clone()))
        .and_then(handler::create_candidate_handler))
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
            .and_then(handler::candidate_list_handler))
        .or((warp::path!("api" / "need"))    
            .and(warp::post())
            .and(warp::body::json())
            .and(with_db(db.clone()))
            .and_then(handler::create_need_handler))
        .or((warp::path!("api" / "need"))
            .and(warp::put())
            .and(warp::path::param())
            .and(warp::body::json())
            .and(with_db(db.clone()))
            .and_then(handler::edit_need_handler))
        .or((warp::path!("api" / "need"))
            .and(warp::delete())
            .and(warp::path::param())
            .and(with_db(db.clone()))
            .and_then(handler::delete_need_handler))
        .or((warp::path!("api" / "need"))
            .and(warp::get())
            .and(with_db(db.clone()))
            .and_then(handler::need_list_handler));

    let routes = candidate_routes.recover(error::handle_rejection);

    println!("Started on port 4000");
    warp::serve(routes).run(([127, 0, 0, 1], 4000)).await;
    Ok(())
}

fn with_db(db: DB) -> impl Filter<Extract = (DB,), Error = Infallible> + Clone {
    warp::any().map(move || db.clone())
}
