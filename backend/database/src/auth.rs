
pub struct DatabaseAuth {
    username: String,
    password: String,
    auth_db: String,
}

impl DatabaseAuth {
    pub fn new(username: String, password: String, auth_db: String) -> DatabaseAuth {
        DatabaseAuth {
            username,
            password,
            auth_db,
        }
    }
}

pub struct DatabaseConnectionData {
    pub(crate) db_name: String,
    pub(crate) db_host: String,
    pub(crate) db_port: u16,
    pub(crate) db_auth: Option<DatabaseAuth>,
}

impl DatabaseConnectionData {
    pub fn new(db_name: String, db_host: String, db_port: u16, db_auth: Option<DatabaseAuth>) -> DatabaseConnectionData {
        DatabaseConnectionData {
            db_name,
            db_host,
            db_port,
            db_auth,
        }
    }

    pub fn get_uri(&self) -> String {
        match &self.db_auth {
            Some(auth) => {
                format!("mongodb://{}:{}@{}:{}/{}?authSource={}", auth.username, auth.password,
                        self.db_host, self.db_port, self.db_name, auth.auth_db)
            },
            None => {
                format!("mongodb://{}:{}/{}", self.db_host, self.db_port, self.db_name)
            }
        }
    }

    pub fn get_db_name(&self) -> &str {
        &self.db_name
    }
}


