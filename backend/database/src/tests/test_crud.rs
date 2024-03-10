use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
struct TestModel {
    id: String,
    age: i32,
    name: String,
}

#[cfg(test)]
mod tests {
    use mongodb::bson::Document;
    use serde::{Serialize, Deserialize};
    use mongodb::Database;
    use crate::connection;
    use crate::connection::get_create_collection;
    use crate::crud;
    use crate::crud::LimitResult;
    use crate::error::DatabaseError;
    use super::TestModel;
    use mongodb::bson::doc;


    #[derive(Serialize, Deserialize)]
    struct Test;

    impl TestModel {
        fn generate_many(amount: u8) -> Vec<TestModel> {
            let mut output: Vec<TestModel> = Vec::new();

            for i in 0..amount {
                let model = TestModel {
                    id: i.to_string(),
                    age: i as i32,
                    name: format!("Name {}", i),
                };

                output.push(model);
            }

            output 
        
        }
    }

    async fn get_database() -> Database {
        std::env::set_var("DATABASE_URL", "mongodb://localhost:27017");
        connection::get_database("test").await.unwrap()
    }

    fn assert_model_equality(
        models_1: Vec<TestModel>, 
        models_2: Vec<TestModel>,
        from_index: usize,
    ) {
        
        let mut model_id = from_index;

        for model in models_2{
            let model_equality = 
                models_1[model_id].id == model.id &&
                models_1[model_id].age ==model.age &&
                models_1[model_id].name == model.name;

            if !model_equality {
                assert!(false, 
                        "Not all models are equal!, failed at id: {}", model_id)
            }
            model_id += 1;
        }

        assert!(true);
    }


    fn validation_rules() -> mongodb::bson::Document {
        doc! {
            "$jsonSchema": {
                "bsonType": "object",
                "required": ["id", "age", "name"],
                "properties": {
                    "id": {
                        "bsonType": "int",
                        "description": "must be a int and is required"
                    },
                    "age": {
                        "bsonType": "int",
                        "description": "must be an integer and is required"
                    },
                    "name": {
                        "bsonType": "string",
                        "description": "must be a string and is required"
                    }
                }
            }
        }
    }
 
    #[tokio::test]
    async fn test_creating_collection() {
        let database = get_database().await;

        let _ = connection::create_collection::<Test>(&database, "test_collection", None).await;
    }
    #[tokio::test]
    async fn test_getting_collection() {
        let database = get_database().await;

        let _ = connection::create_collection::<Test>(&database, "test_another_collection", None).await;
        let _ = connection::get_collection::<Test>(&database, "test_another_collection").await.unwrap();
    }

    #[tokio::test]
    async fn test_saving_model() {
        let database = get_database().await;   
        let collection = connection::get_create_collection(&database, "test_saving_model", None).await;

        let model = TestModel { 
            id: "123".to_string(),
            age: 20,
            name: "John".to_string(),
        };

        let result = crud::save_model(&collection, &model).await;

        match result {
            Ok(_) => assert!(true),
            Err(err) => {
                println!("Error: {}", err);
                assert!(false, "Expected Ok")
            }
        };
        collection.drop(None).await.unwrap();
    }    

    #[tokio::test]
    async fn test_saving_multiple_models() {
        let database = get_database().await;
        let collection = connection::get_create_collection(&database, "test_saving_multiple_models", None).await;

        let models = TestModel::generate_many(10);

        let result = crud::save_multiple_models(&collection, &models).await;


        match result {
            Ok(_) => assert!(true),
            Err(err) => {
                println!("Error: {}", err);
                assert!(false, "Expected Ok")
            }
        };
        collection.drop(None).await.unwrap();
    }

    #[tokio::test]
    async fn test_saving_model_error() {
        let database = get_database().await;

        let validation_rules = validation_rules();
        let collection = get_create_collection(&database, "test_saving_model_error", Some(validation_rules))
                            .await;


        let model = TestModel {
            id: "123".to_string(),
            age: 20,
            name: "John".to_string(),
        };

        let result = crud::save_model(&collection, &model).await;

        match result {
            Ok(_) => assert!(false, "Expected an error"),
            Err(DatabaseError::MongoError(_)) => assert!(true),
            Err(_) => assert!(false, "Expected a MongoError"),
        };
        collection.drop(None).await.unwrap();
    }

    #[tokio::test]
    async fn test_saving_multiple_models_error() {
        let database = get_database().await;
        
        let validation_rules = validation_rules();
        let collection = get_create_collection(&database, "test_saving_multiple_models_error", Some(validation_rules))
                            .await;

        let models = TestModel::generate_many(10);

        let result = crud::save_multiple_models(&collection, &models).await;

        match result {
            Ok(_) => assert!(false, "Expected an error"),
            Err(DatabaseError::MongoError(_)) => assert!(true),
            Err(_) => assert!(false, "Expected a MongoError"),
        };
        collection.drop(None).await.unwrap();
    }

    #[tokio::test]
    async fn test_getting_model() {
        let database = get_database().await;
        let collection = connection::get_create_collection(&database, "test_getting_model", None).await;

        let model = TestModel {
            id: "123".to_string(),
            age: 20,
            name: "John".to_string(),
        };

        crud::save_model(&collection, &model).await.unwrap();

        let result = crud::get_model_by_id(&collection, "123").await;

        match result {
            Ok(Some(model)) => {
                assert_eq!(model.id, "123");
                assert_eq!(model.age, 20);
                assert_eq!(model.name, "John");
            },
            Ok(None) => assert!(false, "Expected a model"),
            Err(err) => {
                println!("Error: {}", err);
                assert!(false, "Expected Ok")
            }
        };
        collection.drop(None).await.unwrap();
    }

    #[tokio::test]
    async fn test_getting_model_returns_none() {
        let database = get_database().await;
        let collection = connection::get_create_collection::<Document>
            (&database, "test_getting_model_returns_none", None).await;

        let result = crud::get_model_by_id(&collection, "123").await;


        match result {
            Ok(None) => assert!(true),
            Ok(Some(_)) => assert!(false, "Expected None"),
            Err(err) => {
                println!("Error: {}", err);
                assert!(false, "Expected Ok")
            }
        };
        collection.drop(None).await.unwrap();
    }

    #[tokio::test]
    async fn test_getting_model_error() {
        let database = get_database().await;

        let validation_rules = validation_rules();
        let collection = connection::get_create_collection(&database, "test_getting_model_error", Some(validation_rules)).await;

        let result = crud::get_model_by_id::<TestModel>
                        (&collection, "123").await;

        match result {
            Ok(_) => assert!(false, "Expected an error"),
            Err(DatabaseError::MongoError(_)) => assert!(true),
            Err(_) => assert!(false, "Expected a MongoError"),
        };

        collection.drop(None).await.unwrap();
    }
    
    #[tokio::test]
    async fn test_get_many_models_with_query_and_limit() {
        let database = get_database().await;
        let collection = connection::get_create_collection
                            (&database, "test_get_many_models_with_query_and_limit", None).await;

        let amount: u8 = 30;
        let models = TestModel::generate_many(amount);


        crud::save_multiple_models(&collection, &models).await.unwrap();
        let query = doc! {"age": doc! {"$gt": 10}};
        let limit = LimitResult::Limited(3);
    
        let result = crud::get_many_models(&collection, Some(query), limit).await;

        match result {
            Ok(Some(obtained_models)) => {
                // because of the query
                // the models obtained have a limit of 3 and a query where 
                // the age > 10, and the age == id
                assert_model_equality(models, obtained_models, 11)
            },
            Ok(None) => assert!(false, "Expected a Some variant"),
            Err(_) => assert!(false, "Expected OK result")
        };
        collection.drop(None).await.unwrap();
    }

    #[tokio::test]
    async fn test_get_many_models_only_limit() {
        let database = get_database().await;
        let collection = connection::get_create_collection
                            (&database, "test_get_many_models_only_limit", None).await;

        let amount: u8 = 30;
        let models = TestModel::generate_many(amount);
        crud::save_multiple_models(&collection, &models).await.unwrap();

        let limit = LimitResult::Limited(3);

        let result = crud::get_many_models(&collection, None, limit).await;

        match result {
            Ok(Some(obtained_models)) => {
                // because of the query
                // the models obtained have a limit of 3 and a query where 
                // the age > 10, and the age == id
                assert_model_equality(models, obtained_models, 0)
            },
            Ok(None) => assert!(false, "Expected a Some variant"),
            Err(_) => assert!(false, "Expected OK result")
        };
        collection.drop(None).await.unwrap();
    }

    #[tokio::test]
    async fn test_get_many_models_no_query_and_limit() {
        let database = connection::get_database("test").await.unwrap();
        let collection = connection::get_create_collection
                            (&database, "test_get_many_models_no_query_and_limit", None).await;

        let amount = 10;
        let models = TestModel::generate_many(amount);

        crud::save_multiple_models(&collection, &models).await.unwrap();

        let limit = LimitResult::Unlimited;
        let result = crud::get_many_models(&collection, None, limit).await;

        match result {
            Ok(Some(obtained_models)) => {
                // because of the query
                // the models obtained have a limit of 3 and a query where 
                // the age > 10, and the age == id
                assert_model_equality(models, obtained_models, 0)
            },
            Ok(None) => assert!(false, "Expected a Some variant"),
            Err(_) => assert!(false, "Expected OK result")
        };
        collection.drop(None).await.unwrap();
    }
}
