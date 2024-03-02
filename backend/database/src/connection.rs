use mongodb::{Client, Database};
use dotenv::dotenv;
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
