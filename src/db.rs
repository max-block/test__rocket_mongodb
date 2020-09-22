use std::error::Error;

use mongodb::{results::InsertOneResult, sync::Client, sync::Collection};
use url::{ Url};
use mongodb::error::Error as MongoError;

use crate::models::Data;

pub struct DB {
    col: Collection,
}

fn parse_database_name_from_url(database_url: &str) -> Result<String, Box<dyn Error>> {
    let parsed = Url::parse(database_url)?;
    Ok(parsed.path()[1..].to_string())
}

impl DB {
    pub fn new(database_url: String) -> Result<DB, Box<dyn Error>> {
        let client = Client::with_uri_str(&database_url)?;
        let database_name = parse_database_name_from_url(&database_url)?;
        let col = client.database(database_name.as_str()).collection("data");
        Ok(DB { col })
    }

    pub fn insert_data(&self, name: String, value: i64, tags: Vec<String>) -> Result<InsertOneResult, MongoError> {
        self.col.insert_one(Data::new(name, value, tags).to_doc(), None)
    }

    pub fn find_data(&self, name: Option<String>, limit: u64) {

    }

    pub fn drop_col(&self) ->Result<(), MongoError>{
        self.col.drop(None)
    }
}
