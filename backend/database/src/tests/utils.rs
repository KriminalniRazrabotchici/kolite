use mongodb::{bson::{doc, Document}, Database};

use crate::{auth::DatabaseConnectionData, handler::DatabaseHandler};

pub fn sample_validation_rules() -> Document {
    let validation_schema: Document = doc! {
        "$jsonSchema": {
            "bsonType": "object",
            "required": ["id", "name", "age"],
            "properties": {
                "id": {
                    "bsonType": "string",
                    "description": "must be a string and is required",
                    "minLength": 1,
                },
                "name": {
                    "bsonType": "string",
                    "description": "must be a string and is required",
                    "minLength": 1,
                },
                "age": {
                    "bsonType": "int",
                    "minimum": 0,
                    "maximum": 120,
                    "description": "must be an integer in [0, 120] and is required",
                },
            }
        }
    };

   validation_schema 
} 


pub async fn get_database_handler() -> DatabaseHandler {
    let db_name = "test".to_string();
    let host = "localhost".to_string();
    let port = 27017;
    let conn_data = DatabaseConnectionData::new(db_name, host, port, None);

    DatabaseHandler::new(conn_data).await.unwrap()
}
    
pub async fn get_database() -> Database {
    let database_handler = get_database_handler().await;

    database_handler.get_database()
}
