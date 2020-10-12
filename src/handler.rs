use crate::{db::DB, WebResult};
use serde::{Deserialize, Serialize};
use warp::{http::StatusCode, reject, reply::json, Reply};


#[derive(Serialize, Deserialize, Debug)]
pub struct CandidateRequest {
    pub first_name: String,
    pub last_name: String,
    pub status_candidate: String,
    pub status_index:String,
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
pub struct NeedRequest {
  pub  post_name: String,
  pub  location:String,
  pub  customer: String,
  pub  experience: String,
  pub  max_salary: String,
  pub  start_date: String,
  pub  creation_date: String,
  pub  manager_name: String,
  pub  cr_name: String,
  pub  reference_need: String,
  pub  status_need: String,
  pub  status_index: String,
  pub  affected_candidat_list: Vec<String>,
  pub  tags: Vec<String>,
}

/// structure request config
// pub struct Follow_ConfigRequest{
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
pub async fn candidate_list_handler(db: DB) -> WebResult<impl Reply> {
    let candidates = db.fetch_candidate().await.map_err(|e| reject::custom(e))?;
    Ok(json(&candidates))
}

pub async fn create_candidate_handler(body: CandidateRequest, db: DB) -> WebResult<impl Reply> {
    db.create_candidate(&body).await.map_err(|e| reject::custom(e))?;
    Ok(StatusCode::CREATED)
}

pub async fn edit_candidate_handler(id: String, body: CandidateRequest, db: DB) -> WebResult<impl Reply> {
    db.edit_candidate(&id, &body)
        .await
        .map_err(|e| reject::custom(e))?;
    Ok(StatusCode::OK)
}

pub async fn delete_candidate_handler(id: String, db: DB) -> WebResult<impl Reply> {
    db.delete_candidate(&id).await.map_err(|e| reject::custom(e))?;
    Ok(StatusCode::OK)
}

pub async fn need_list_handler(db: DB) -> WebResult<impl Reply> {
    let needs = db.fetch_need().await.map_err(|e| reject::custom(e))?;
    Ok(json(&needs))
}

pub async fn create_need_handler(body: NeedRequest, db: DB) -> WebResult<impl Reply> {
    db.create_need(&body).await.map_err(|e| reject::custom(e))?;
    Ok(StatusCode::CREATED)
}

pub async fn edit_need_handler(id: String, body: NeedRequest, db: DB) -> WebResult<impl Reply> {
    db.edit_need(&id, &body)
        .await
        .map_err(|e| reject::custom(e))?;
    Ok(StatusCode::OK)
}

pub async fn delete_need_handler(id: String, db: DB) -> WebResult<impl Reply> {
    db.delete_need(&id).await.map_err(|e| reject::custom(e))?;
    Ok(StatusCode::OK)
}

