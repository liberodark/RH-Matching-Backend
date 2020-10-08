
use super::super::{DB,WebResult};
use serde::{Deserialize, Serialize};
use warp::{http::StatusCode, reject, reply::json, Reply};
#[derive(Serialize, Deserialize, Debug)]

pub struct CandidateRequest {
    pub firstName: String,
    pub lastName: String,
    pub statusCandidate: String,
    pub statusIndex:usize,
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
    pub cv: String,
    pub needReference: Vec<String>,
    pub needReferenceId: Vec<String>,
    pub comment: String,
    pub mobility: String,
    pub tags: Vec<String>,

}

pub async fn candidate_list_handler(db: DB) -> WebResult<impl Reply> {
    let candidates = db.fetch_candidates().await.map_err(|e| reject::custom(e))?;
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
