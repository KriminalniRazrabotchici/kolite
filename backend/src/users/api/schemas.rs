use serde::{Deserialize, Serialize};
use crate::auth::AuthenticationDetails;

use crate::users::User;


#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub(super) message: String
}

impl ErrorResponse {
    pub fn new(message: &str) -> Self {
        ErrorResponse {
            message: message.to_string()
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUser {
    username: String,
    email: String,
    password: String,
}

impl Into<User> for CreateUser {
    fn into(self) -> User {
        User::new(&self.username, &self.password, &self.email)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginUserData {
    pub(super) email: String,
    pub(super) password: String
}


#[derive(Debug, Serialize, Deserialize)]
pub struct SuccessfulLoginResponse {
    pub(super) message: String,
    pub(super) access_token: String,
    pub(super) refresh_token: String
}

impl SuccessfulLoginResponse {
    pub fn new(message: &str, auth_details: AuthenticationDetails) -> Self {
        SuccessfulLoginResponse {
            message: message.to_string(),
            access_token: auth_details.get_token().to_string(), 
            refresh_token: auth_details.get_refresh_token().to_string()
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Refresh {
    pub(super) refresh_token: String
}
