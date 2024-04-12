use mongodb::bson::Document;
use mongodb::options::{CreateCollectionOptions, ValidationAction, ValidationLevel};
use mongodb::{Client, Database};
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::auth::DatabaseConnectionData;
use crate::collections::CollectionHandler;
use crate::errors::DatabaseError; 


pub struct DatabaseHandler {
    client: Client,
    database: Database,
}

impl DatabaseHandler  {
    pub async fn new(connection_data: DatabaseConnectionData) -> Result<DatabaseHandler, DatabaseError> {
        let uri = connection_data.get_uri();

        let client_result = Client::with_uri_str(&uri).await;
        
        let client = match client_result {
            Ok(client) => client,
            Err(e) => return Err(DatabaseError::ConnectionError(e.to_string())),
        };

        match DatabaseHandler::test_connection(&client).await {
            Ok(_) => (),
            Err(e) => return Err(e),
            
        };

        let database = client.database(connection_data.get_db_name());
        let handler = DatabaseHandler {
            client: client,
            database,
        };
        Ok(handler)
    }

    async fn test_connection(client: &Client) -> Result<(), DatabaseError>{
        match client.list_database_names(None, None).await {
            Ok(_) => Ok(()),
            Err(e) => Err(DatabaseError::ConnectionError(e.to_string())),
        }
    }

    pub fn get_database(&self) -> Database {
        self.database.clone()
    }

    pub fn get_client(&self) -> Client {
        self.client.clone()
    }

    pub async fn create_collection(&self, collection_name: &str, validation_rules: Option<Document>) -> Result<(), DatabaseError> {
        let options = CreateCollectionOptions::builder()
            .validator(validation_rules)
            .validation_level(ValidationLevel::Moderate)
            .validation_action(ValidationAction::Error)
            .build();

        match self.database.create_collection(collection_name, options).await {
            Ok(_) => Ok(()),
            Err(e) => Err(DatabaseError::CreateCollectionError(e.to_string())),
        }
        
    }

    pub async fn get_collection<T>(&self, collection_name: &str) -> Result<CollectionHandler<T>, DatabaseError> 
    where T: Serialize + DeserializeOwned
    {
        let collections = self.database.list_collection_names(None).await
            .map_err(|e| DatabaseError::MongoError(e))?;

        if !collections.contains(&collection_name.to_string()) {
            return Err(DatabaseError::CreateCollectionError(format!("Collection {} does not exist", collection_name)));
        }

        let collection = self.database.collection::<T>(collection_name);

        Ok(CollectionHandler::from(collection))
    }
}
