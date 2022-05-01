use crate::shortener;
use anyhow::Error;
use bson::doc;
use mongodb::bson::oid::ObjectId;
use mongodb::bson::{DateTime, Document};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Entry {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub created_at: DateTime,
    pub short: String,
    pub url: String,
}

// Keys
const ID: &str = "_id";
const CREATED_AT: &str = "created_at";
const SHORT_URL: &str = "short";
const URL: &str = "url";

impl Entry {
    pub fn new(hash: String, url: String) -> Entry {
        Entry {
            id: None,
            created_at: DateTime::now().to_owned(),
            short: hash,
            url,
        }
    }

    pub fn from_url(url: String) -> Entry {
        let hash = shortener::make(url.as_str());
        Entry::new(hash, url)
    }

    pub fn from_doc(doc: &Document) -> Result<Entry, Error> {
        let id = Some(doc.get_object_id(ID)?);
        let created_at = doc.get_datetime(CREATED_AT)?;
        let short_url = doc.get_str(SHORT_URL)?;
        let url = doc.get_str(URL)?;

        let entry = Entry {
            id: id.to_owned(),
            created_at: created_at.to_owned(),
            short: short_url.to_owned(),
            url: url.to_owned(),
        };

        Ok(entry)
    }
}
