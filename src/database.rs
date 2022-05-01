use crate::entry::Entry;
use anyhow::Error;
use mongodb::bson::doc;
use mongodb::{Client, Collection};

const DB_NAME: &str = "shorturlDB";
const COLL_NAME: &str = "urls";

#[derive(Clone)]
pub struct Database {
    pub client: Client,
}

// Decodes a base64_url string to string.
fn decode_to_string(base64: &str) -> Result<String, Error> {
    let decoded = base64_url::decode(base64)?;
    let str = String::from_utf8_lossy(decoded.as_slice()).parse()?;
    Ok(str)
}

impl Database {
    // Tries to create new client with given connection string.
    pub async fn new(connection_string: &str) -> Result<Database, Error> {
        let client = Client::with_uri_str(connection_string).await?;
        let db_conn = Database { client };
        Ok(db_conn)
    }

    // Handles /api/s/{base64}
    pub async fn get_shortened(&self, base64: String) -> Result<Entry, Error> {
        let collection = self.get_collection();

        // Decode the URL from Base64URL
        let url = decode_to_string(base64.as_str())?;
        println!("searching for {url}");

        // Search inside the collection.
        let filter = doc! { "url": url.clone()};
        let result = collection.find_one(filter, None).await?;

        return match result {
            // Return the existing URL
            Some(val) => {
                println!("Found entry in database {:?}", val);
                Ok(val)
            }
            None => {
                println!("Creating new entry");
                let created = self.insert_new(url).await?;
                Ok(created)
            }
        };
    }

    pub async fn get_unrolled(&self, hash: String) -> Result<Entry, Error> {
        let collection = self.get_collection();

        let filter = doc! { "short": hash.clone()};
        let result = collection.find_one(filter, None).await?;

        return match result {
            Some(val) => {
                println!("Hash {hash} exists in the database");
                Ok(val)
            }
            None => Err(Error::msg("Hash does not exist in the database")),
        };
    }

    // Returns the URLs collection.
    fn get_collection(&self) -> Collection<Entry> {
        self.client.database(DB_NAME).collection(COLL_NAME)
    }

    // Creates the entry and inserts it to DB.
    async fn insert_new(&self, url: String) -> Result<Entry, Error> {
        let collection = self.get_collection();
        let created = Entry::from_url(url);

        let insert_result = collection.insert_one(created.clone(), None).await?;
        let id = insert_result
            .inserted_id
            .as_object_id()
            .expect("Retrieved _id should have been of type ObjectId");
        println!("Inserted ID: {:?}", id);

        Ok(created)
    }
}
