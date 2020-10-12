use crate::{
    error::Error::*, handler::CandidateRequest, handler::NeedRequest, Candidate, Need, Result,
};
use chrono::prelude::*;
use futures::StreamExt;
use mongodb::bson::{doc, document::Document, oid::ObjectId, Bson};
use mongodb::{options::ClientOptions, Client, Collection};

#[derive(Clone, Debug)]
pub struct DB {
    pub client: Client,
}
// config DB
const COLL_NEED: &str = "need";
const COLL_CANDIDATE: &str = "candidate";
//const COLL_CONFIG: &str = "follow-config";
const DB_NAME: &str = "follow";

//candidate constante
const ID_CANDIDATE: &str = "_id";
const FIRST_NAME: &str = "first_name";
const LAST_NAME: &str = "last_name";
const STATUS_CANDIDATE: &str = "status_candidate";
const STATUS_INDEX_CANDIDATE: &str = "status_index";
const STATUS_DATE: &str = "status_date";
const EMAIL: &str = "e_mail";
const PHONE_NUMBER: &str = "phone_number";
const POST_TITLE: &str = "post_title";
const ORIGIN: &str = "origin";
const CUSTOMER: &str = "customer";
const EXPERIENCE: &str = "experience_candidate";
const SALARY: &str = "salary";
const AVAILABILITY_DATE: &str = "availability_date";
const MANAGER_NAME_CANDIDATE: &str = "manger_name";
const CR_NAME_CANDIDATE: &str = "cr_name";
const KOTAG: &str = "ko_tag";
const CV_CANDIDAT: &str = "cv_candidat";
const NEED_REFERENCE: &str = "need_reference";
const NEED_REFERENCE_ID: &str = "need_reference_id";
const COMMENT: &str = "comment";
const MOBILITY: &str = "mobility";
const TAGS_CANDIDATE: &str = "tags";

// need const
const ID_NEED: &str = "id";
const POST_NAME: &str = "post_name";
const LOCATION: &str = "location";
const CUSTOMER_NEED: &str = "customer";
const EXPERIENCE_NEED: &str = "experience";
const MAX_SALARY: &str = "max_salary";
const START_DATE: &str = "start_date";
const CREATION_DATE: &str = "creation_date";
const MANAGER_NAME_NEED: &str = "manager_name";
const CR_NAME_NEED: &str = "cr_name";
const REFERENCE_NEED: &str = "reference";
const STATUS_NEED: &str = "status_need";
const STATUS_INDEX_NEED: &str = "status_index";
const AFFECTED_CANDIDAT_LIST: &str = "affected_candidat_list";
const TAGS_NEED: &str = "tags";

//follow config Const
//   const  NEED_STATUS_CONFIG: &str = "need_Status";
//   const  CANDIDATE_STATUS_CONFIG: &str = "candidate_Status";
//   const  CR_NAMES_CONFIG: &str = "Cr_Names";
//   const  MANAGER_NAME_CONFIG: &str = "Manager_Name";
//   const  CUSTOMER_CONFIG: &str = "customer";
//   const  MOBILITY_CONFIG: &str = "mobility";
//   const  EXPERIENCE_CONFIG: &str = "experience";
//   const  KOTAG_CONFIG: &str = "ko_tag";
//   const  ORIGIN_CONFIG: &str = "origin";

impl DB {
    // setup db
    pub async fn init() -> Result<Self> {
        // let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let mut client_options = ClientOptions::parse("mongodb://127.0.0.1:27017").await?;
        client_options.app_name = Some("follow".to_string());

        Ok(Self {
            client: Client::with_options(client_options)?,
        })
    }

    // get all candidates
    pub async fn fetch_candidate(&self) -> Result<Vec<Candidate>> {
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

    pub async fn edit_candidate(&self, id: &str, entry: &CandidateRequest) -> Result<()> {
        let oid = ObjectId::with_string(id).map_err(|_| InvalidIDError(id.to_owned()))?;
        let query = doc! {
            "_id": oid,
        };
        let doc = doc! {
          
            FIRST_NAME: entry.first_name.clone(),
            LAST_NAME: entry.last_name.clone(),
            STATUS_CANDIDATE: entry.status_candidate.clone(),
            STATUS_INDEX_CANDIDATE: entry.status_index.clone(),
            STATUS_DATE: entry.status_date.clone(),
            EMAIL: entry.e_mail.clone(),
            PHONE_NUMBER: entry.phone_number.clone(),
            POST_TITLE: entry.post_title.clone(),
            ORIGIN: entry.origin.clone(),
            CUSTOMER: entry.customer.clone(),
            EXPERIENCE: entry.experience_candidate.clone(),
            SALARY: entry.salary.clone(),
            AVAILABILITY_DATE: entry.availability_date.clone(),
            MANAGER_NAME_CANDIDATE: entry.manger_name.clone(),
            CR_NAME_CANDIDATE: entry.cr_name.clone(),
            KOTAG: entry.ko_tag.clone(),
            CV_CANDIDAT: entry.cv_candidat.clone(),
            NEED_REFERENCE: entry.need_reference.clone(),
            NEED_REFERENCE_ID: entry.need_reference_id.clone(),
            COMMENT: entry.comment.clone(),
            MOBILITY: entry.mobility.clone(),
            TAGS_CANDIDATE: entry.tags.clone(),
        };
        self.get_collection_candidate()
            .update_one(query, doc, None)
            .await
            .map_err(MongoQueryError)?;
        Ok(())
    }

    pub async fn create_candidate(&self, entry: &CandidateRequest) -> Result<()> {
        let doc = doc! {
            FIRST_NAME: entry.first_name.clone(),
            LAST_NAME: entry.last_name.clone(),
            STATUS_CANDIDATE: entry.status_candidate.clone(),
            STATUS_INDEX_CANDIDATE: entry.status_index.clone(),
            STATUS_DATE: entry.status_date.clone(),
            EMAIL: entry. e_mail.clone(),
            PHONE_NUMBER: entry.phone_number.clone(),
            POST_TITLE: entry.post_title.clone(),
            ORIGIN: entry.origin.clone(),
            CUSTOMER: entry.customer.clone(),
            EXPERIENCE: entry.experience_candidate.clone(),
            SALARY: entry.salary.clone(),
            AVAILABILITY_DATE: entry.availability_date.clone(),
            MANAGER_NAME_CANDIDATE: entry.manger_name.clone(),
            CR_NAME_CANDIDATE: entry.cr_name.clone(),
            KOTAG: entry.ko_tag.clone(),
            CV_CANDIDAT: entry.cv_candidat.clone(),
            NEED_REFERENCE: entry.need_reference.clone(),
            NEED_REFERENCE_ID: entry.need_reference_id.clone(),
            COMMENT: entry.comment.clone(),
            MOBILITY: entry.mobility.clone(),
            TAGS_CANDIDATE: entry.tags.clone(),
        };

        self.get_collection_candidate()
            .insert_one(doc, None)
            .await
            .map_err(MongoQueryError)?;
        Ok(())
    }

    pub async fn delete_candidate(&self, id: &str) -> Result<()> {
        let oid = ObjectId::with_string(id).map_err(|_| InvalidIDError(id.to_owned()))?;
        let filter = doc! {
            "_id": oid,
        };

        self.get_collection_candidate()
            .delete_one(filter, None)
            .await
            .map_err(MongoQueryError)?;
        Ok(())
    }

    fn get_collection_candidate(&self) -> Collection {
        self.client.database(DB_NAME).collection(COLL_CANDIDATE)
    }

    fn doc_to_candidate(&self, doc: &Document) -> Result<Candidate> {
        let id = doc.get_object_id(ID_CANDIDATE)?;
        let first_name = doc.get_str(FIRST_NAME)?;
        let last_name = doc.get_str(LAST_NAME)?;
        let status_candidate = doc.get_str(STATUS_CANDIDATE)?;
        let status_index = doc.get_str(STATUS_INDEX_CANDIDATE)?;
        let status_date = doc.get_str(STATUS_DATE)?;
        let e_mail = doc.get_str(EMAIL)?;
        let phone_number = doc.get_str(PHONE_NUMBER)?;
        let post_title = doc.get_str(POST_TITLE)?;
        let origin = doc.get_str(ORIGIN)?;
        let customer = doc.get_str(CUSTOMER)?;
        let experience_candidate = doc.get_str(EXPERIENCE)?;
        let salary = doc.get_str(SALARY)?;
        let availability_date = doc.get_str(AVAILABILITY_DATE)?;
        let manger_name = doc.get_array(MANAGER_NAME_CANDIDATE)?;
        let cr_name = doc.get_str(CR_NAME_CANDIDATE)?;
        let ko_tag = doc.get_str(KOTAG)?;
        let cv_candidat = doc.get_str(CV_CANDIDAT)?;
        let need_reference = doc.get_array(NEED_REFERENCE)?;
        let need_reference_id = doc.get_array(NEED_REFERENCE_ID)?;
        let comment = doc.get_str(COMMENT)?;
        let mobility = doc.get_str(MOBILITY)?;
        let tags = doc.get_array(TAGS_CANDIDATE)?;

        let candidate = Candidate {
            id: id.to_hex(),
            first_name: first_name.to_owned(),
            last_name: last_name.to_owned(),
            status_candidate: status_candidate.to_owned(),
            status_index: status_index.to_owned(),
            status_date: status_date.to_owned(),
            e_mail: e_mail.to_owned(),
            phone_number: phone_number.to_owned(),
            post_title: post_title.to_owned(),
            origin: origin.to_owned(),
            customer: customer.to_owned(),
            experience_candidate: experience_candidate.to_owned(),
            salary: salary.to_owned(),
            availability_date: availability_date.to_owned(),
            manger_name: manger_name
                .iter()
                .filter_map(|entry| match entry {
                    Bson::String(v) => Some(v.to_owned()),
                    _ => None,
                })
                .collect(),
            cr_name: cr_name.to_owned(),
            ko_tag: ko_tag.to_owned(),
            cv_candidat: cv_candidat.to_owned(),
            need_reference: need_reference
                .iter()
                .filter_map(|entry| match entry {
                    Bson::String(v) => Some(v.to_owned()),
                    _ => None,
                })
                .collect(),
            need_reference_id: need_reference_id
                .iter()
                .filter_map(|entry| match entry {
                    Bson::String(v) => Some(v.to_owned()),
                    _ => None,
                })
                .collect(),
            comment: comment.to_owned(),
            mobility: mobility.to_owned(),
            tags: tags
                .iter()
                .filter_map(|entry| match entry {
                    Bson::String(v) => Some(v.to_owned()),
                    _ => None,
                })
                .collect(),
            
        };
        Ok(candidate)
    }

    
    pub async fn fetch_need(&self) -> Result<Vec<Need>> {
        let mut cursor = self
            .get_collection_candidate()
            .find(None, None)
            .await
            .map_err(MongoQueryError)?;
        let mut result: Vec<Need> = Vec::new();
        while let Some(doc) = cursor.next().await {
            result.push(self.doc_to_need(&doc?)?);
        }
        Ok(result)
    }

    pub async fn edit_need(&self, id: &str, entry: &NeedRequest) -> Result<()> {
        let oid = ObjectId::with_string(id).map_err(|_| InvalidIDError(id.to_owned()))?;
        let query = doc! {
            "_id": oid,
        };
        let doc = doc! {
            POST_NAME : entry.post_name.clone(),
            LOCATION: entry.location.clone(),
            CUSTOMER: entry.customer.clone(),
            EXPERIENCE_NEED: entry.experience.clone(),
            MAX_SALARY: entry.max_salary.clone(),
            START_DATE: entry.start_date.clone(),
            CREATION_DATE: entry.creation_date.clone(),
            MANAGER_NAME_NEED: entry.manager_name.clone(),
            CR_NAME_NEED: entry.cr_name.clone(),
            REFERENCE_NEED: entry.reference_need.clone(),
            STATUS_NEED: entry.status_need.clone(),
            STATUS_INDEX_NEED: entry.status_index.clone(),
            AFFECTED_CANDIDAT_LIST: entry.affected_candidat_list.clone(),
            TAGS_NEED: entry.tags.clone(),
        };
        self.get_collection_need()
            .update_one(query, doc, None)
            .await
            .map_err(MongoQueryError)?;
        Ok(())
    }

    pub async fn create_need(&self, entry: &NeedRequest) -> Result<()> {
        let doc = doc! {

            POST_NAME : entry.post_name.clone(),
            LOCATION: entry.location.clone(),
            CUSTOMER: entry.customer.clone(),
            EXPERIENCE_NEED: entry.experience.clone(),
            MAX_SALARY: entry.max_salary.clone(),
            START_DATE: entry.start_date.clone(),
            CREATION_DATE: entry.creation_date.clone(),
            MANAGER_NAME_NEED: entry.manager_name.clone(),
            CR_NAME_NEED: entry.cr_name.clone(),
            REFERENCE_NEED: entry.reference_need.clone(),
            STATUS_NEED: entry.status_need.clone(),
            STATUS_INDEX_NEED: entry.status_index.clone(),
            AFFECTED_CANDIDAT_LIST: entry.affected_candidat_list.clone(),
            TAGS_NEED: entry.tags.clone(),
        };

        self.get_collection_need()
            .insert_one(doc, None)
            .await
            .map_err(MongoQueryError)?;
        Ok(())
    }

    pub async fn delete_need(&self, id: &str) -> Result<()> {
        let oid = ObjectId::with_string(id).map_err(|_| InvalidIDError(id.to_owned()))?;
        let filter = doc! {
            "_id": oid,
        };

        self.get_collection_need()
            .delete_one(filter, None)
            .await
            .map_err(MongoQueryError)?;
        Ok(())
    }

    fn get_collection_need(&self) -> Collection {
        self.client.database(DB_NAME).collection(COLL_NEED)
    }

    fn doc_to_need(&self, doc: &Document) -> Result<Need> {
        let id = doc.get_object_id(ID_NEED)?;
        let post_name = doc.get_str(POST_NAME)?;
        let location = doc.get_str(LOCATION)?;
        let customer = doc.get_str(CUSTOMER_NEED)?;
        let experience = doc.get_str(EXPERIENCE_NEED)?;
        let max_salary = doc.get_str(MAX_SALARY)?;
        let start_date = doc.get_str(START_DATE)?;
        let creation_date = doc.get_str(CREATION_DATE)?;
        let manager_name = doc.get_str(MANAGER_NAME_NEED)?;
        let cr_name = doc.get_str(CR_NAME_NEED)?;
        let reference_need = doc.get_str(REFERENCE_NEED)?;
        let status_need = doc.get_str(STATUS_NEED)?;
        let status_index = doc.get_str(STATUS_INDEX_NEED)?;
        let affected_candidat_list =  doc.get_array(AFFECTED_CANDIDAT_LIST)?;
        let tags =  doc.get_array(TAGS_NEED)?;

        let need = Need {
            id: id.to_hex(),
            post_name: post_name.to_owned(),
            location: location.to_owned(),
            customer: customer.to_owned(),
            experience: experience.to_owned(),
            max_salary: max_salary.to_owned(),
            start_date:start_date.to_owned(),
            creation_date: creation_date.to_owned(),
            manager_name: manager_name.to_owned(),
            reference_need: reference_need.to_owned(),
            cr_name: cr_name.to_owned(),
            status_need: status_need.to_owned(),
            status_index: status_index.to_owned(),
            affected_candidat_list: affected_candidat_list
            .iter()
                .filter_map(|entry| match entry {
                    Bson::String(v) => Some(v.to_owned()),
                    _ => None,
                })
                .collect(),
            tags:tags
            .iter()
                .filter_map(|entry| match entry {
                    Bson::String(v) => Some(v.to_owned()),
                    _ => None,
                })
                .collect(),
        };
        Ok(need)
    }
}
