#[cfg(test)]
mod test_collection_handler {
    use mongodb::bson::Document;
    use crate::tests::utils::{get_database, get_database_handler, sample_validation_rules};
    use crate::collections::CollectionHandler;



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

    #[tokio::test]
    async fn test_creating_collection_with_validation() {
        let handler = get_database_handler().await;

        let validation = sample_validation_rules();

        let creation_result = handler.create_collection("test_collection_with_validation", Some(validation)).await;

        match creation_result {
            Ok(_) => assert!(true),
            Err(_) => assert!(false),
        }
    }

}
