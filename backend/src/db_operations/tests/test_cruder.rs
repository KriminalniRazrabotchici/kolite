#[cfg(test)]
mod tests {
    use database::{auth::DatabaseConnectionData, handler::DatabaseHandler};

    use crate::db_operations::cruder::Cruder;

    async fn get_db_handler() -> DatabaseHandler {
        let db_conn = DatabaseConnectionData::new("test_db".to_string(), "localhost".to_string(), 27017, None); 

        DatabaseHandler::new(db_conn).await.unwrap()
    }
    
    #[tokio::test]
    async fn test_creating_cruder() {
        let db_handler = get_db_handler().await; 

        let cruder = Cruder::from(db_handler);

        assert_eq!(cruder.get_db_handler().get_database().name(), "test_db");
    }

    async fn test_saving_user_model() {
        todo!()
    }
}
