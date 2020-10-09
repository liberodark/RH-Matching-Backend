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
#[macro_use]

// config DB
const COLL_NEED: &str = "need";
const COLL_CANDIDATE: &str = "candidate";
//const COLL_CONFIG: &str = "follow-config";
const DB_NAME: &str = "follow";

//candidate constante
const ID_CANDIDATE: &str = "_id";
const FIRST_NAME: &str = "firstName";
const LAST_NAME: &str = "lastName";
const STATUS_CANDIDATE: &str = "statusCandidate";
const STATUS_INDEX_CANDIDATE: &str = "statusIndex";
const STATUS_DATE: &str = "statusDate";
const EMAIL: &str = "Email";
const PHONE_NUMBER: &str = "phoneNumber";
const POST_TITLE: &str = "postTitle";
const ORIGIN: &str = "origin";
const CUSTOMER: &str = "customer";
const EXPERIENCE: &str = "experience";
const SALARY: &str = "salary";
const AVAILABILITY_DATE: &str = "availabilityDate";
const MANAGER_NAME_CANDIDATE: &str = "mangerName";
const CR_NAME_CANDIDATE: &str = "CrName";
const KOTAG: &str = "KoTag";
const CV_CANDIDAT: &str = "cvCandidat";
const NEED_REFERENCE: &str = "needReference";
const NEED_REFERENCE_ID: &str = "needReferenceId";
const COMMENT: &str = "comment";
const MOBILITY: &str = "mobility";
const TAGS_CANDIDATE: &str = "tags";

// need const
const ID_NEED: &str = "id";
const POST_NAME: &str = "postName";
const LOCATION: &str = "location";
const CUSTOMER_NEED: &str = "customer";
const EXPERIENCE_NEED: &str = "experience";
const MAX_SALARY: &str = "maxSalary";
const START_DATE: &str = "StartDate";
const CREATION_DATE: &str = "creationDate";
const MANAGER_NAME_NEED: &str = "managerName";
const CR_NAME_NEED: &str = "CrName";
const REFERENCE_NEED: &str = "reference";
const STATUS_NEED: &str = "statusNeed";
const STATUS_INDEX_NEED: &str = "statusIndex";
const AFFECTED_CANDIDAT_LIST: &str = "affectedCandidatList";
const TAGS_NEED: &str = "tags";

//follow config Const
//   const  NEED_STATUS_CONFIG: &str = "need_Status";
//   const  CANDIDATE_STATUS_CONFIG: &str = "candidate_Status";
//   const  CR_NAMES_CONFIG: &str = "Cr_Names";
//   const  MANAGER_NAME_CONFIG: &str = "Manager_Name";
//   const  CUSTOMER_CONFIG: &str = "customer";
//   const  MOBILITY_CONFIG: &str = "mobility";
//   const  EXPERIENCE_CONFIG: &str = "experience";
//   const  KOTAG_CONFIG: &str = "KoTag";
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
          
            FIRST_NAME: entry.firstName.clone(),
            LAST_NAME:  entry.lastName.clone(),
            STATUS_CANDIDATE: entry.statusCandidate.clone(),
            STATUS_INDEX_CANDIDATE: entry.statusIndex.clone(),
            STATUS_DATE: entry.statusDate.clone(),
            EMAIL: entry.Email.clone(),
            PHONE_NUMBER: entry.phoneNumber.clone(),
            POST_TITLE: entry.postTitle.clone(),
            ORIGIN: entry.origin.clone(),
            CUSTOMER: entry.customer.clone(),
            EXPERIENCE: entry.experience.clone(),
            SALARY: entry.salary.clone(),
            AVAILABILITY_DATE: entry.availabilityDate.clone(),
            MANAGER_NAME_CANDIDATE: entry.mangerName.clone(),
            CR_NAME_CANDIDATE: entry.CrName.clone(),
            KOTAG: entry.KoTag.clone(),
            CV_CANDIDAT: entry.cvCandidat.clone(),
            NEED_REFERENCE: entry.needReference.clone(),
            NEED_REFERENCE_ID: entry.needReferenceId.clone(),
            COMMENT: entry.comment.clone(),
            MOBILITY:entry.mobility.clone(),
            TAGS_CANDIDATE:  entry.tags.clone(),
        };
        self.get_collection_candidate()
            .update_one(query, doc, None)
            .await
            .map_err(MongoQueryError)?;
        Ok(())
    }

    pub async fn create_candidate(&self, entry: &CandidateRequest) -> Result<()> {
        let doc = doc! {
            FIRST_NAME: entry.firstName.clone(),
            LAST_NAME:  entry.lastName.clone(),
            STATUS_CANDIDATE: entry.statusCandidate.clone(),
            STATUS_INDEX_CANDIDATE: entry.statusIndex.clone(),
            STATUS_DATE: entry.statusDate.clone(),
            EMAIL: entry.Email.clone(),
            PHONE_NUMBER: entry.phoneNumber.clone(),
            POST_TITLE: entry.postTitle.clone(),
            ORIGIN: entry.origin.clone(),
            CUSTOMER: entry.customer.clone(),
            EXPERIENCE: entry.experience.clone(),
            SALARY: entry.salary.clone(),
            AVAILABILITY_DATE: entry.availabilityDate.clone(),
            MANAGER_NAME_CANDIDATE: entry.mangerName.clone(),
            CR_NAME_CANDIDATE: entry.CrName.clone(),
            KOTAG: entry.KoTag.clone(),
            CV_CANDIDAT: entry.cvCandidat.clone(),
            NEED_REFERENCE: entry.needReference.clone(),
            NEED_REFERENCE_ID: entry.needReferenceId.clone(),
            COMMENT: entry.comment.clone(),
            MOBILITY:entry.mobility.clone(),
            TAGS_CANDIDATE:  entry.tags.clone(),
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
        let firstName = doc.get_str(FIRST_NAME)?;
        let lastName = doc.get_str(LAST_NAME)?;
        let statusCandidate = doc.get_str(STATUS_CANDIDATE)?;
        let statusIndex = doc.get_str(STATUS_INDEX_CANDIDATE)?;
        let statusDate = doc.get_str(STATUS_DATE)?;
        let Email = doc.get_str(EMAIL)?;
        let phoneNumber = doc.get_str(PHONE_NUMBER)?;
        let postTitle = doc.get_str(POST_TITLE)?;
        let origin = doc.get_str(ORIGIN)?;
        let customer = doc.get_str(CUSTOMER)?;
        let experience = doc.get_str(EXPERIENCE)?;
        let salary = doc.get_str(SALARY)?;
        let availabilityDate = doc.get_str(AVAILABILITY_DATE)?;
        let mangerName = doc.get_array(MANAGER_NAME_CANDIDATE)?;
        let CrName = doc.get_str(CR_NAME_CANDIDATE)?;
        let KoTag = doc.get_str(KOTAG)?;
        let CvCandidat = doc.get_str(CV_CANDIDAT)?;
        let needReference = doc.get_array(NEED_REFERENCE)?;
        let needReferenceId = doc.get_array(NEED_REFERENCE_ID)?;
        let comment = doc.get_str(COMMENT)?;
        let mobility = doc.get_str(MOBILITY)?;
        let tags = doc.get_array(TAGS_CANDIDATE)?;

        let candidate = Candidate {
            id: id.to_hex(),
            firstName: firstName.to_owned(),
            lastName: lastName.to_owned(),
            statusCandidate: statusCandidate.to_owned(),
            statusIndex: statusIndex.to_owned(),
            statusDate: statusDate.to_owned(),
            Email: Email.to_owned(),
            phoneNumber: phoneNumber.to_owned(),
            postTitle: postTitle.to_owned(),
            origin: origin.to_owned(),
            customer: customer.to_owned(),
            experience: experience.to_owned(),
            salary: salary.to_owned(),
            availabilityDate: availabilityDate.to_owned(),
            mangerName: mangerName
                .iter()
                .filter_map(|entry| match entry {
                    Bson::String(v) => Some(v.to_owned()),
                    _ => None,
                })
                .collect(),
            CrName: CrName.to_owned(),
            KoTag: KoTag.to_owned(),
            cvCandidat: CvCandidat.to_owned(),
            needReference: needReference
                .iter()
                .filter_map(|entry| match entry {
                    Bson::String(v) => Some(v.to_owned()),
                    _ => None,
                })
                .collect(),
            needReferenceId: needReferenceId
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

    
    pub async fn fetch_Need(&self) -> Result<Vec<Need>> {
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
            POST_NAME : entry.postName.clone(),
            LOCATION: entry.location.clone(),
            CUSTOMER: entry.customer.clone(),
            EXPERIENCE_NEED: entry.experience.clone(),
            MAX_SALARY: entry.maxSalary.clone(),
            START_DATE: entry.StartDate.clone(),
            CREATION_DATE: entry.creationDate.clone(),
            MANAGER_NAME_NEED: entry.managerName.clone(),
            CR_NAME_NEED: entry.CrName.clone(),
            REFERENCE_NEED: entry.referenceNeed.clone(),
            STATUS_NEED: entry.statusNeed.clone(),
            STATUS_INDEX_NEED: entry.statusIndex.clone(),
            AFFECTED_CANDIDAT_LIST: entry.affectedCandidatList.clone(),
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

            POST_NAME : entry.postName.clone(),
            LOCATION: entry.location.clone(),
            CUSTOMER: entry.customer.clone(),
            EXPERIENCE_NEED: entry.experience.clone(),
            MAX_SALARY: entry.maxSalary.clone(),
            START_DATE: entry.StartDate.clone(),
            CREATION_DATE: entry.creationDate.clone(),
            MANAGER_NAME_NEED: entry.managerName.clone(),
            CR_NAME_NEED: entry.CrName.clone(),
            REFERENCE_NEED: entry.referenceNeed.clone(),
            STATUS_NEED: entry.statusNeed.clone(),
            STATUS_INDEX_NEED: entry.statusIndex.clone(),
            AFFECTED_CANDIDAT_LIST: entry.affectedCandidatList.clone(),
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
        let postName = doc.get_str(POST_NAME)?;
        let location = doc.get_str(LOCATION)?;
        let customer = doc.get_str(CUSTOMER_NEED)?;
        let experience = doc.get_str(EXPERIENCE_NEED)?;
        let maxSalary = doc.get_str(MAX_SALARY)?;
        let StartDate = doc.get_str(START_DATE)?;
        let creationDate = doc.get_str(CREATION_DATE)?;
        let managerName = doc.get_str(MANAGER_NAME_NEED)?;
        let CrName = doc.get_str(CR_NAME_NEED)?;
        let referenceNeed = doc.get_str(REFERENCE_NEED)?;
        let statusNeed = doc.get_str(STATUS_NEED)?;
        let statusIndex = doc.get_str(STATUS_INDEX_NEED)?;
        let affectedCandidatList =  doc.get_array(AFFECTED_CANDIDAT_LIST)?;
        let tags =  doc.get_array(TAGS_NEED)?;

        let need = Need {
            id: id.to_hex(),
            postName: postName.to_owned(),
            location: location.to_owned(),
            customer: customer.to_owned(),
            experience: experience.to_owned(),
            maxSalary: maxSalary.to_owned(),
            StartDate:StartDate.to_owned(),
            creationDate: creationDate.to_owned(),
            managerName: managerName.to_owned(),
            referenceNeed: referenceNeed.to_owned(),
            CrName: CrName.to_owned(),
            statusNeed: statusNeed.to_owned(),
            statusIndex: statusIndex.to_owned(),
            affectedCandidatList: affectedCandidatList
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
