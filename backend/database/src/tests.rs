use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
struct TestModel {
    id: String,
    age: i32,
    name: String,
}



#[cfg(test)]
mod tests {
    use mockall::predicate::*;
    use crate::connection;
    use crate::crud;
    use crate::error::DatabaseError;
    use std::sync::Mutex;
    use super::TestModel;
    use mongodb::bson::doc;

    pub(crate) static TEST_MUTEX: Mutex<()> = Mutex::new(());
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

    fn assert_model_equality(models_1: Vec<TestModel>, models_2: Vec<TestModel>) {
        let even_length = models_1.len() == models_2.len();

        assert!(even_length);

        for model_id in 0..models_1.len(){
            let model_equality = 
                models_1[model_id].id == models_2[model_id].id &&
                models_1[model_id].age == models_2[model_id].age &&
                models_1[model_id].name == models_2[model_id].name;

            if !model_equality {
                assert!(false, 
                        "Not all models are equal!, failed at id: {}", model_id)
            }
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

    async fn apply_validation_rules(database: &mongodb::Database, collection_name: &str) -> Result<(), DatabaseError> {
        let validation = validation_rules();
        
        let options = mongodb::options::CreateCollectionOptions::builder()
            .validator(validation)
            .validation_action(mongodb::options::ValidationAction::Error)
            .build();

        database.create_collection(collection_name, options).await.map_err(|err| {
            DatabaseError::MongoError(err)
        })?;


        Ok(())
    }
        
    fn prepare_test() {
        let _lock = TEST_MUTEX.lock().unwrap();
        std::env::set_var("DATABASE_URL", "mongodb://localhost:27017");
    }

    fn prepare_wrong_env_test_error() {
        let _lock = TEST_MUTEX.lock().unwrap();
        std::env::set_var("DATABASE_URL", "not_mongo:://localhost:27017");
    }
    
    #[tokio::test]
    async fn test_database_connection() {
        prepare_test();
        let _ = connection::get_database("test").await.unwrap();
    }

    #[tokio::test]
    async fn test_database_connection_error() {
        prepare_wrong_env_test_error();
        let database = connection::get_database("test").await;

        match database {
            Ok(_) => assert!(false, "Expected an error"),
            Err(DatabaseError::MongoError(_)) => assert!(true),
            Err(_) => assert!(false, "Expected a MongoError"),
        };
    }

    #[tokio::test]
    async fn test_saving_model() {
        prepare_test();
        let database = connection::get_database("test").await.unwrap();  

        let model = TestModel { 
            id: "123".to_string(),
            age: 20,
            name: "John".to_string(),
        };

        let result = crud::save_model(&database, "test", model).await;

        match result {
            Ok(_) => assert!(true),
            Err(err) => {
                println!("Error: {}", err);
                assert!(false, "Expected Ok")
            }
        };

        database.drop(None).await.unwrap();
    }    

    #[tokio::test]
    async fn test_saving_multiple_models() {
        prepare_test();
        let database = connection::get_database("test").await.unwrap();

        let models = TestModel::generate_many(10);

        let result = crud::save_multiple_models(&database, "test", models).await;


        match result {
            Ok(_) => assert!(true),
            Err(err) => {
                println!("Error: {}", err);
                assert!(false, "Expected Ok")
            }
        };

        database.drop(None).await.unwrap();
    }

    #[tokio::test]
    async fn test_saving_model_error() {
        prepare_test();
        
        let database = connection::get_database("test").await.unwrap();  

        apply_validation_rules(&database, "wrong").await.unwrap();

        let model = TestModel {
            id: "123".to_string(),
            age: 20,
            name: "John".to_string(),
        };

        let result = crud::save_model(&database, "wrong", model).await;

        match result {
            Ok(_) => assert!(false, "Expected an error"),
            Err(DatabaseError::MongoError(_)) => assert!(true),
            Err(_) => assert!(false, "Expected a MongoError"),
        };

        database.drop(None).await.unwrap();
    }

    #[tokio::test]
    async fn test_saving_multiple_models_error() {
        prepare_test();
        let database = connection::get_database("test").await.unwrap();

        apply_validation_rules(&database, "wrong").await.unwrap();

        let models = TestModel::generate_many(10);

        let result = crud::save_multiple_models(&database, "wrong", models).await;

        match result {
            Ok(_) => assert!(false, "Expected an error"),
            Err(DatabaseError::MongoError(_)) => assert!(true),
            Err(_) => assert!(false, "Expected a MongoError"),
        };

        database.drop(None).await.unwrap();
    }

    #[tokio::test]
    async fn test_getting_model() {
        prepare_test();
        let database = connection::get_database("test").await.unwrap();

        let model = TestModel {
            id: "123".to_string(),
            age: 20,
            name: "John".to_string(),
        };

        crud::save_model(&database, "test", model).await.unwrap();

        let result = crud::get_model_by_id
            ::<TestModel>(&database, "test", "123").await;        

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

        database.drop(None).await.unwrap();
    }

    #[tokio::test]
    async fn test_getting_model_returns_none() {
        prepare_test();
        let database = connection::get_database("none").await.unwrap();

        let result = crud::get_model_by_id
            ::<TestModel>(&database, "test", "123").await;


        match result {
            Ok(None) => assert!(true),
            Ok(Some(_)) => assert!(false, "Expected None"),
            Err(err) => {
                println!("Error: {}", err);
                assert!(false, "Expected Ok")
            }
        };

        database.drop(None).await.unwrap();
    }

    // #[tokio::test]
    // async fn test_getting_model_error() {
    //     prepare_test();
    //     let database = connection::get_database("test").await.unwrap();
    //
    //     let model = TestModel {
    //         id: "123".to_string(),
    //         age: 20,
    //         name: "John".to_string(),
    //     };
    //
    //     crud::save_model(&database, "test", model).await.unwrap();
    //     
    //
    //     let result = crud::get_model_by_id::<TestModel>
    //         (&database, "wrong", "123").await;
    //
    //     match result {
    //         Ok(_) => assert!(false, "Expected an error"),
    //         Err(DatabaseError::MongoError(_)) => assert!(true),
    //         Err(_) => assert!(false, "Expected a MongoError"),
    //     };
    //
    //     database.drop(None).await.unwrap();
    // }
    
    #[tokio::test]
    async fn get_many_models() {
        prepare_test();
        let database = connection::get_database("test").await.unwrap();


        let amount: u8 = 30;
        let models = TestModel::generate_many(amount);

        crud::save_multiple_models(&database, "test_many", models).await.unwrap();
        let query = doc! {"id": doc! {"$gt": 10}};
        let limit = 5;

        let result = crud::get_many_models::<TestModel>(&database, query, limit).await;

        match result {
            Ok(Some(result_models)) => assert_model_equality(models, result_models),
            Ok(None) => assert!(false, "Expected a Some variant"),
            Err(_) => assert!(false, "Expected OK result")
        }
    }


}
