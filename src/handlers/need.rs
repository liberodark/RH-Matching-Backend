

use crate::{db::DB, WebResult};
use serde::{Deserialize, Serialize};
use warp::{http::StatusCode, reject, reply::json, Reply};
use chrono::{DateTime,Utc,naive}

#[derive(Serialize, Deserialize, Debug)]
pub struct NeedRequest {
    id: String,
    postName: String,
    location:String,
    customer: String,
    experience: String,
    maxSalary: u64,
    StartDate: DateTime::<Utc>,
    creationDate: DateTime::<Utc>,
    managerName: String,
    CR_NAME: String,
    referenceNeed: String,
    TechnoSystem: String,
    statusName: String,
    statusIndex:usize,
    affectedCandidatList: Vec<String>,
}

pub async fn need_list_handler(db: DB) -> WebResult<impl Reply> {
    let needs = db.fetch_needs().await.map_err(|e| reject::custom(e))?;
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