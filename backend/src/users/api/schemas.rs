use serde::{Deserialize, Serialize};

use crate::users::User;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUser {
    username: String,
    email: String,
    password: String,
}

impl Into<User> for CreateUser {
    fn into(self) -> User {
        User::new(&self.username, &self.email, &self.password)
    }
}
