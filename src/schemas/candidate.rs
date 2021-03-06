use crate::{db::DB,Result};
use mongodb::bson::{doc, document::Document, oid::ObjectId, Bson};

#[derive(Serialize, Deserialize, Debug)]
pub struct Candidate {
    pub id:String,
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

const COLL: &str = "candidates";
const ID: &str = "_id";
const FIRST_NAME: &str = "firstName";
const LAST_NAME: &str = "lastName";
const STATUS_CANDIDATE: &str ="statusCandidate";
const STATUS_INDEX: &str="statusIndex",
const STATUS_DATE: &str = "statusDate";
const EMAIL: &str = "Email";
const PHONE_NUMBER: &str = "phoneNumber";
const POST_TITLE: &str = "postTitle";
const ORIGIN: &str = "origin";
const CUSTOMER: &str= "customer";
const EXPERIENCE: &str = "experience";
const SALARY: &str = "salary";
const AVAILABILITY_DATE: &str = "availabilityDate";
const MANAGER_NAME: &str = "mangerName";
const CR_NAME: &str = "CrName";
const KOTAG: &str = "KoTag";
const CV:&str = "Cv";
const NEED_REFERENCE: &str = "needReference";
const NEED_REFERENCE_ID: &str = "needReferenceId";
const COMMENT: &str = "comment";
const MOBILITY: &str = "mobility";
const TAGS: &str = "tags";

impl Candidate {
    fn doc_to_candidate(db::DB, doc: &Document) -> Result<Candidate> {
        let id = doc.get_object_id(ID)?;
        let firstName = doc.get_str(FIRSTNAME)?;
        let lastName = doc.get_str(LASTNAME)?;
        let statusCandidate = doc.get_str(STATUS_CANDIDATE)?;
        let statusIndex = doc.get_str(STATUS_INDEX);
        let statusDate = doc.get_str(STATUS_DATE);
        let Email = doc.get_str(EMAIL);
        let phoneNumber=doc.get_str(PHONE_NUMBER);
        let postTitle = doc.get_str(POST_TITLE);
        let origin = doc.get_str(ORIGIN);
        let customer = doc.get_str(CUSTOMER);
        let experience =  doc.get_str(EXPERIENCE);        
        let salary = doc.get_i32(SALARY)?;
        let availabilityDate =  doc.get_str(AVAILABILITY_DATE);
        let mangerName = doc.get_str(MANAGER_NAME);
        let CrName = doc.get_str(CR_NAME);
        let KoTag = doc.get_str(KOTAG);
        let Cv = doc.get_str(CV);
        let needReference = doc.get_str(NEED_REFERENCE);
        let needReferenceId = doc.get_str(NEED_REFERENCE_ID);
        let comment = doc.get_str(COMMENT);
        let tags = doc.get_array(TAGS)?;

        let candidate = Candidate {
            id: id.to_hex(),
            firstName: firstName.to_owned(),
            lastName: lastName.to_owned(),
            statusCandidate: statusCandidate.to_owned(),
            statusIndex:usize,
            statusDate: statusDate.to_owned(),
            Email: Email.to_owned(),
            phoneNumber: phoneNumber.to_owned(),
            postTitle: postTitle,
            origin: origin,
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
            CrName:  CrName,
            KoTag: KoTag,
            cv: cv,
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
            comment: comment,
            mobility:  mobility.to_owned(),
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
}