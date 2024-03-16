#[cfg(test)]
mod tests {
    use crate::auth::DatabaseAuth;
    use crate::auth::DatabaseConnectionData;

    #[test]
    fn test_create_conn_data_no_auth() {
        let db_name = "test".to_string();
        let db_host = "localhost".to_string();
        let db_port = 27017;

        let conn_data = DatabaseConnectionData::new(db_name.clone(), db_host, db_port, None);

        assert_eq!(conn_data.db_name, db_name);
    } 

    #[test]
    fn test_create_conn_data_with_auth() {
        let db_name = "test".to_string();
        let db_host = "localhost".to_string();
        let db_port = 27017;
        let db_auth = DatabaseAuth::new("admin".to_string(), "test".to_string(), "test".to_string());

        let conn_data = DatabaseConnectionData::new(db_name.clone(), db_host, db_port, Some(db_auth));

        assert_eq!(conn_data.db_name, db_name);
    }
}
