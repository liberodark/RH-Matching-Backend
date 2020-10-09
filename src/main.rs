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
    pub id: String,
    pub postName: String,
    pub location:String,
    pub customer: String,
    pub experience: String,
    pub maxSalary: String,
    pub StartDate: String,
    pub creationDate: String,
    pub managerName: String,
    pub CrName: String,
    pub referenceNeed: String,
    pub statusNeed: String,
    pub statusIndex:String,
    pub affectedCandidatList: Vec<String>,
    pub tags: Vec<String>,
}

//config Structure 
// pub struct Follow_Config{
//    pub need_Status:  Vec<String>,
//    pub candidate_Status: Vec<String>,
//    pub Cr_Names: Vec<String>,
//    pub Manager_Name: Vec<String>,
//    pub customer: Vec<String>, 
//    pub mobility: Vec<String>,
//    pub experience: Vec<String>,
//    pub KoTag: Vec<String>,
//    pub origin: Vec<String>,
// }
#[tokio::main]
async fn main() -> Result<()> {
    let db = DB::init().await?;
    // let options = warp::options()
    //     .and(warp::header::<String>("origin")).map(|origin| {
    //         Ok(http::Response::builder()
    //             .header("access-control-allow-methods", "HEAD, GET, POST, DELETE, PUT")
    //             .header("access-control-allow-headers", "authorization")
    //             .header("access-control-allow-credentials", "true")
    //             .header("access-control-max-age", "300")
    //             .header("access-control-allow-origin", origin)
    //             .header("vary", "origin")
    //             .body(""))
    // });
    
   // let candidate_path = warp::path("api/V1");
    let all_routes = 
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
            .and_then(handler::need_list_handler))
        .with(warp::cors()
        .allow_any_origin()
        .allow_headers(vec!["User-Agent", "Sec-Fetch-Mode", "Referer", "Origin", "Access-Control-Request-Method", "Access-Control-Request-Headers"])
        .allow_methods(vec!["POST", "GET","DELETE"])
        );

    let routes = all_routes.recover(error::handle_rejection);

    println!("Started on port 4000");
    warp::serve(routes).run(([127, 0, 0, 1], 4000)).await;
    Ok(())
}

fn with_db(db: DB) -> impl Filter<Extract = (DB,), Error = Infallible> + Clone {
    warp::any().map(move || db.clone())
}
