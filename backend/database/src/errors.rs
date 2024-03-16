use std::fmt;

#[derive(Debug)]
pub enum DatabaseError {
    MongoError(mongodb::error::Error),
    ConnectionError(String),
    AuthError(String),
}

impl fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DatabaseError::MongoError(e) => write!(f, "MongoError: {}", e),
            DatabaseError::ConnectionError(e) => write!(f, "ConnectionError: {}", e),
            DatabaseError::AuthError(e) => write!(f, "AuthError: {}", e),
        }
    }
}
