use mongodb::{options::ClientOptions, Client, Collection};
use std::env;
mod schemas;
#[derive(Clone, Debug)]
pub struct DB {
    pub client: Client,
}
#[macro_use]

const COLL_NEED:&str = "need";
const COLL_CANDIDATE:&str = "candidate";
const DB_NAME:&str= "follow";

impl DB {
    pub async fn init() -> Result<Self> {
        // let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let mut client_options = ClientOptions::parse("mongodb://127.0.0.1:27017").await?;
        client_options.app_name = Some("follow".to_string());

        Ok(Self {
            client: Client::with_options(client_options)?,
        })
    }
    fn get_collection_candidate(&self) -> Collection {
        self.client.database(DB_NAME).collection(COLL_CANDIDATE)
    }
    pub async fn fetch_candidates(&self) -> Result<Vec<Candidate>> {
        let mut cursor = self
            .get_collection_candidate()
            .find(None, None)
            .await
            .map_err(MongoQueryError)?;

        let mut result: Vec<Candidate> = Vec::new();
        while let Some(doc) = cursor.next().await {
            result.push(self.doc_to_candidate(&doc?)?);
        }
        Ok(result)
    }
}
