use mongodb::{Client, Database};

use crate::auth::DatabaseConnectionData;
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
}
