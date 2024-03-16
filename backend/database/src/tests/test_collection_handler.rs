pub struct SampleModel {
    id: String,
    name: String,
    age: u8,
}


impl SampleModel {
    pub fn new(id: String, name: String, age: u8) -> SampleModel {
        SampleModel {
            id,
            name,
            age,
        }
    }

    pub fn generate_many(amount: u8) -> Vec<SampleModel> {
        let mut models = Vec::new();

        for i in 0..amount {
            let model = SampleModel::new(i.to_string(), "John".to_string(), i);
            models.push(model);
        }

        models
    }
}


#[cfg(test)]
mod test_collection_handler {
    use mongodb::bson::{doc, Document};
    use mongodb::Database;

    use super::SampleModel;

    use crate::{auth::DatabaseConnectionData, handler::DatabaseHandler};
    use crate::collections::CollectionHandler;

    fn assert_models_are_equal(models: Vec<SampleModel>, get_result: Vec<SampleModel>) {
        for i in 0..models.len() {
            assert_eq!(models[i].id, get_result[i].id);
            assert_eq!(models[i].name, get_result[i].name);
            assert_eq!(models[i].age, get_result[i].age);
        }
    }

    fn sample_validation_rules() -> Document {
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

    #[tokio::test]
    async fn test_inserting_model_to_database() {
        let handler = get_database_handler().await;

        let collection_handler = handler.get_collection::<Document>("test_inserting_model_to_database").await.unwrap();

        let model = SampleModel::new("1".to_string(), "John".to_string(), 20);


        let insert_result = collection_handler.save_one(&model).await;

        match insert_result {
            Ok(_) => assert!(true),
            Err(_) => assert!(false),
        }
    }

    #[tokio::test]
    async fn test_inserting_many_models_to_database() {
        let handler = get_database_handler().await;

        let collection_handler = handler.get_collection::<Document>("test_inserting_many_models_to_database").await.unwrap();

        let models = SampleModel::generate_many(10);

        let insert_result = collection_handler.save_many(&models).await;

        match insert_result {
            Ok(_) => assert!(true),
            Err(_) => assert!(false),
        }
    }


    #[tokio::test]
    async fn test_getting_many_models_from_database() {
        let handler = get_database_handler().await;

        let collection_handler = handler.get_collection::<Document>("test_getting_many_models_from_database").await.unwrap();

        let models = SampleModel::generate_many(10);

        let _ = collection_handler.save_many(&models).await;

        let returned_models = collection_handler.get_many::<SampleModel>(None, None).await.unwrap();

        assert_models_are_equal(models, returned_models);
    }


    #[tokio::test]
    async fn test_getting_many_models_with_query_and_limit() {
        let handler = get_database_handler().await;

        let collection_handler = handler.get_collection::<Document>("test_getting_many_models_with_query").await.unwrap();

        let models = SampleModel::generate_many(10);

        let _ = collection_handler.save_many(&models).await;

        let query = doc! {
            "age": {
                "$gt": 5,
            }
        };

        let limit_result = LimitResult::Limited(5);

        let returned_models = collection_handler.get_many::<SampleModel>(Some(query), None).await.unwrap();

        let expected_models: Vec<SampleModel> = models.into_iter().filter(|model| model.age > 5).collect();

        assert_models_are_equal(expected_models, returned_models);
    }
}
