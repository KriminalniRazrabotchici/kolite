use core::fmt;

use database::errors::DatabaseError;


#[derive(Debug)]
pub enum CrudError {
    DatabaseError (DatabaseError),
    CustomError (String)
}


impl fmt::Display for CrudError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CrudError::DatabaseError(e) => e.fmt(f),
            CrudError::CustomError(e) => write!(f, "{}", e),
        }
    }
}
