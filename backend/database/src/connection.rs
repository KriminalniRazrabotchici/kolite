use mongodb::{bson::Document, options::{CreateCollectionOptions, ValidationAction, ValidationLevel}, Client, Database, Collection};
use dotenv::dotenv;
use serde::{de::DeserializeOwned, Serialize};
use core::panic;
use std::env;
use super::error::DatabaseError;

pub async fn connect() -> Result<Client, mongodb::error::Error> {
    let _ = dotenv();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let client = Client::with_uri_str(database_url).await?;

    Ok(client)
}

pub async fn get_database(database_name: &str) -> Result<Database, DatabaseError> {
    let client = connect().await.map_err(|err| {
        DatabaseError::MongoError(err)
    })?;

    Ok(client.database(database_name))
}

pub(crate) async fn get_collection<T>(database: &Database, collection_name: &str,) -> Option<Collection<T>> 
where T:
    Serialize + DeserializeOwned
{

    let collection_results = database.list_collection_names(None).await;

    let collections = match collection_results {
        Err(_) => return None,
        Ok(collections) => collections
    };

    for current_name in collections.iter() {
        if current_name == collection_name{
            let collection = database.collection(collection_name);
            return Some(collection);
        }
    }

    None
}

pub(crate) async fn create_collection<T>(database: &Database, collection_name: &str, validation_rules: Option<Document>) -> Collection<T> 
where T:
    Serialize + DeserializeOwned
{
    let options = CreateCollectionOptions::builder()
                    .validator(validation_rules)
                    .validation_action(ValidationAction::Error)
                    .validation_level(ValidationLevel::Moderate)
                    .build();

    let creation_result = database.create_collection(collection_name, options).await;

    match creation_result {
        Err(_) => panic!("Unable to create collection {}", collection_name),
        Ok(_) => database.collection(collection_name)
    } 
}

pub async fn get_create_collection<T>(database: &Database, collection_name: &str, validation_rules: Option<Document>) -> Collection<T> 
where T:
    Serialize + DeserializeOwned
{
    match get_collection(database, collection_name).await {
        Some(collection) => collection,
        None => create_collection(database, collection_name, validation_rules).await
    }
}
