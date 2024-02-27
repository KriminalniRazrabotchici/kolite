use mockall::*;
use mockall::predicate::*;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::database::connection;
    use crate::database::error::DatabaseError;
    use mongodb::Database;
    
    #[actix_web::test]
    async fn test_database_connection() {
        std::env::set_var("DATABASE_URL", "mongodb://localhost:27017");
        let _ = connection::get_database("test").await.unwrap();
        std::env::remove_var("DATABASE_URL");
    }

    #[actix_web::test]
    async fn test_database_connection_error() {
        std::env::set_var("DATABASE_URL", "not_mongo:://localhost:27017");
        let database = connection::get_database("test").await;

        std::env::remove_var("DATABASE_URL");

        match database {
            Ok(_) => assert!(false, "Expected an error"),
            Err(DatabaseError::MongoError(_)) => assert!(true),
            Err(_) => assert!(false, "Expected a MongoError"),
        }
    }

    
}

