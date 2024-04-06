use core::fmt;

#[derive(Debug)]
pub enum JWTError {
    NativeError(jsonwebtoken::errors::Error),
    CustomError(String)
}

impl JWTError { 
    pub fn custom(message: &str) -> JWTError {
        JWTError::CustomError(message.to_string())
    } 

    pub fn native(error: jsonwebtoken::errors::Error) -> JWTError {
        JWTError::NativeError(error)
    }
}

impl From<jsonwebtoken::errors::Error> for JWTError {
    fn from(value: jsonwebtoken::errors::Error) -> Self { 
        JWTError::native(value)
    }
}

impl fmt::Display for JWTError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JWTError::NativeError(e) => write!(f, "JWTError: {}", e),
            JWTError::CustomError(e) => write!(f, "JWTError: {}", e)
        }
    }
}
