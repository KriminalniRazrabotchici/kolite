use std::fmt::Display;

use mongodb::error::Error as MongoError;

pub enum DatabaseError {
    MongoError(MongoError),
    CustomError(String),
}

impl DatabaseError {
    pub fn custom(err: &str) -> Self {
        DatabaseError::CustomError(err.to_string())
    }
}


impl From<MongoError> for DatabaseError {
    fn from(err: MongoError) -> Self {
        DatabaseError::MongoError(err)
    }
}

impl Display for DatabaseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DatabaseError::MongoError(err) => write!(f, "MongoError: {}", err),
            DatabaseError::CustomError(err) => write!(f, "CustomError: {}", err),
        }
    }
}
