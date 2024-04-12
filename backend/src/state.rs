use std::env;

use database::{auth::{DatabaseAuth, DatabaseConnectionData}, handler::DatabaseHandler};

use crate::db_operations::cruder::Cruder;

pub struct AppState {
    cruder: Cruder,
}

impl AppState {
    pub async fn init() -> AppState {
        let conn_data = AppState::create_database_connection_data();
        let db = DatabaseHandler::new(conn_data).await.unwrap();

        let cruder = Cruder::new(db);

        AppState {
            cruder
        }
    } 

    fn create_database_connection_data() -> DatabaseConnectionData {
        let _ = dotenv::dotenv();
        let username = env::var("DB_USER").unwrap();
        let pass = env::var("DB_PASS").unwrap();
        let auth_db = env::var("DB_AUTH").unwrap();

        // username, pass, auth_db
        let auth = DatabaseAuth::new(username, pass, auth_db);


        let db_name = env::var("DB_NAME").unwrap();
        let db_host = env::var("DB_HOST").unwrap();
        let db_port: u16 = env::var("DB_PORT").unwrap()
            .parse().unwrap();

        // db_name, db_host, db_port, db_auth 
        DatabaseConnectionData::new(db_name, db_host, db_port, Some(auth))
    }

    pub fn get_cruder(&self) -> &Cruder {
        &self.cruder
    }


}
