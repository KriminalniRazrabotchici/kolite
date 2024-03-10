#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use crate::error::DatabaseError;
    use crate::connection;

    pub(crate) static TEST_LOCK: Mutex<()> = Mutex::new(());

        
    #[tokio::test(flavor="current_thread")]
    async fn test_database_connection() {
        let _lock = TEST_LOCK.lock().unwrap();
        std::env::set_var("DATABASE_URL", "mongodb://localhost:27017");

        let database = connection::get_database("test").await.unwrap();
        
        database.drop(None).await.unwrap();
    }

    #[tokio::test(flavor="current_thread")]
    async fn test_database_connection_error() {
        let _lock = TEST_LOCK.lock().unwrap();
        std::env::set_var("DATABASE_URL", "Total giberish"); 

        let database = connection::get_database("test").await;

        match database {
            Ok(_) => assert!(false, "Expected an error"),
            Err(DatabaseError::MongoError(_)) => assert!(true),
            Err(_) => assert!(false, "Expected a MongoError"),
        };
    }

}
