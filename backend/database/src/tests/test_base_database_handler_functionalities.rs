#[cfg(test)]
mod tests {
    use mongodb::Database;

    use crate::{auth::{DatabaseAuth, DatabaseConnectionData}, handler::DatabaseHandler};


    #[tokio::test(flavor = "current_thread")]
    async fn test_creating_database_handler() {
        let db_name = "test".to_string();
        let db_host = "localhost".to_string();
        let db_port = 27017;

        let database_conn = DatabaseConnectionData::new(db_name.clone(), db_host, db_port, None);

        let _ = DatabaseHandler::new(database_conn).await.unwrap();


        assert!(true);
    }

    #[tokio::test(flavor = "current_thread")]
    async fn test_creating_database_with_auth_fails() {
        let db_name = "test_auth".to_string();
        let db_host = "localhost".to_string();
        let db_port = 27017; 

        let auth = DatabaseAuth::new("admin".to_string(), "test".to_string(), "test".to_string());
        let database_conn = DatabaseConnectionData::new(db_name.clone(), db_host, db_port, Some(auth));

        let handler = DatabaseHandler::new(database_conn).await;


        match handler {
            Ok(_) => assert!(false),
            Err(_) => assert!(true),
        }
    }

    #[tokio::test]
    async fn test_getting_the_database() {
        let db_name = "test".to_string();
        let db_host = "localhost".to_string();
        let db_port = 27017;

        let connection_inf = DatabaseConnectionData::new(db_name.clone(), db_host, db_port, None);

        let db_handler = DatabaseHandler::new(connection_inf).await.unwrap();

        let db: Database = db_handler.get_database();

        assert_eq!(db_name, db.name());
    }

    #[tokio::test]
    async fn test_getting_the_client() {
        let db_name = "test".to_string();
        let db_host = "localhost".to_string();
        let db_port = 27017;

        let connection_inf = DatabaseConnectionData::new(db_name.clone(), db_host, db_port, None);

        let db_handler = DatabaseHandler::new(connection_inf).await.unwrap();

        let client = db_handler.get_client();

        assert_eq!(db_name, client.database("test").name());
    }
}
