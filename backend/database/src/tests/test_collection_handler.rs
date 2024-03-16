
#[cfg(test)]
mod test_collection_handler {
    use mongodb::bson::Document;
    use mongodb::Database;

    use crate::{auth::DatabaseConnectionData, handler::DatabaseHandler};
    use crate::collections::CollectionHandler;


    async fn get_database_handler() -> DatabaseHandler {
        let db_name = "test".to_string();
        let host = "localhost".to_string();
        let port = 27017;
        let conn_data = DatabaseConnectionData::new(db_name, host, port, None);

        DatabaseHandler::new(conn_data).await.unwrap()
    }
        
    async fn get_database() -> Database {
        let database_handler = get_database_handler().await;
    
        database_handler.get_database()
    }

    #[tokio::test]
    async fn test_creating_handler() {
        let database = get_database().await;

        let collection = database.collection::<Document>("test_collection");

        let _ = CollectionHandler::from(collection);

        assert!(true);
    }

    #[tokio::test]
    async fn test_creating_collection_using_db_handler() {
        let handler = get_database_handler().await;

        let creation_result = handler.create_collection("test_creating_collection", None).await;

        match creation_result {
            Ok(_) => assert!(true),
            Err(_) => assert!(false),
        }
    }

    #[tokio::test]
    async fn test_creating_collection_handler_for_existing_collection() {
        let handler = get_database_handler().await;

        let creation_result = handler.create_collection("test_existing_collection", None).await;
        let collection_handler = handler.get_collection::<Document>("test_existing_collection").await;

        assert!(creation_result.is_ok());
        
        match collection_handler {
            Ok(_) => assert!(true),
            Err(_) => assert!(false),
        }
    }

    #[tokio::test]
    async fn test_creating_collection_handler_for_non_existing_collection() {
        let handler = get_database_handler().await;

        let collection_handler = handler.get_collection::<Document>("non_existing_collection").await;
        
        match collection_handler {
            Ok(_) => assert!(false),
            Err(_) => assert!(true),
        }
    }
}
