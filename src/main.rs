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
    pub first_name: String,
    pub last_name: String,
    pub status_candidate: String,
    pub status_index: String,
    pub status_date: String,
    pub e_mail: String,
    pub phone_number: String,
    pub post_title: String,
    pub origin: String,
    pub customer: String,
    pub experience_candidate: String,
    pub salary: String,
    pub availability_date: String,
    pub manger_name: Vec<String>,
    pub cr_name: String,
    pub ko_tag: String,
    pub cv_candidat: String,
    pub need_reference: Vec<String>,
    pub need_reference_id: Vec<String>,
    pub comment: String,
    pub mobility: String,
    pub tags: Vec<String>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Need {
    pub id: String,
    pub post_name: String,
    pub location:String,
    pub customer: String,
    pub experience: String,
    pub max_salary: String,
    pub start_date: String,
    pub creation_date: String,
    pub manager_name: String,
    pub cr_name: String,
    pub reference_need: String,
    pub status_need: String,
    pub status_index:String,
    pub affected_candidat_list: Vec<String>,
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
//    pub ko_tag: Vec<String>,
//    pub origin: Vec<String>,
// }
#[tokio::main]
async fn main() -> Result<()> {
    let db = DB::init().await?;
    let all_routes = 
        ((warp::path!("api" / "candidate"))
        .and(warp::post())
        .and(warp::body::json())
        .and(with_db(db.clone()))
        .and_then(handler::create_candidate_handler))
        .or((warp::path!("api" / "candidate"))
            .and(warp::put())
            .and(warp::path::param())
            .and(warp::body::json())
            .and(with_db(db.clone()))
            .and_then(handler::edit_candidate_handler))
        .or((warp::path!("api" / "candidate"))
            .and(warp::delete())
            .and(warp::path::param())
            .and(with_db(db.clone()))
            .and_then(handler::delete_candidate_handler))
        .or((warp::path!("api" / "candidate"))
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
        .allow_methods(vec!["POST", "GET","DELETE","PUT"]));

    let routes = all_routes.recover(error::handle_rejection);

    println!("Started on port 4000");
    warp::serve(routes).run(([127, 0, 0, 1], 4000)).await;
    Ok(())
}

fn with_db(db: DB) -> impl Filter<Extract = (DB,), Error = Infallible> + Clone {
    warp::any().map(move || db.clone())
}
