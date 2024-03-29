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
