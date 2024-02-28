use std::sync::Mutex;

use mockall::*;
use mockall::predicate::*;
use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
struct TestModel {
    id: String,
    age: i32,
    name: String,
}

pub(crate) static TEST_MUTEX: Mutex<()> = Mutex::new(());

#[cfg(test)]
mod tests {
    use super::*;
    use crate::database::connection;
    use crate::database::error::DatabaseError;
    use crate::database::crud;

    fn prepare_test() {
        let _lock = super::TEST_MUTEX.lock().unwrap();
        std::env::set_var("DATABASE_URL", "mongodb://localhost:27017");
    }

    fn prepare_wrong_env_test_error() {
        let _lock = super::TEST_MUTEX.lock().unwrap();
        std::env::set_var("DATABASE_URL", "not_mongo:://localhost:27017");
    }
    
    #[actix_web::test]
    async fn test_database_connection() {
        prepare_test();
        let _ = connection::get_database("test").await.unwrap();
    }

    #[actix_web::test]
    async fn test_database_connection_error() {
        prepare_wrong_env_test_error();
        let database = connection::get_database("test").await;

        match database {
            Ok(_) => assert!(false, "Expected an error"),
            Err(DatabaseError::MongoError(_)) => assert!(true),
            Err(_) => assert!(false, "Expected a MongoError"),
        }
    }

    #[actix_web::test]
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
        }
    }    

    #[actix_web::test]
    async fn test_saving_model_error() {
        prepare_test();
        let _lock = super::TEST_MUTEX.lock().unwrap();
        std::env::set_var("DATABASE_URL", "mongodb://localhost:27017");
        let database = connection::get_database("test").await.unwrap();  

        let model = TestModel {
            id: "123".to_string(),
            age: 20,
            name: "John".to_string(),
        };

        let result = crud::save_model(&database, "test", model).await;

        match result {
            Ok(_) => assert!(false, "Expected an error"),
            Err(DatabaseError::MongoError(_)) => assert!(true),
            Err(_) => assert!(false, "Expected a MongoError"),
        }
    }

}
