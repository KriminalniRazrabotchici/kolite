use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    username: String,
    email: String,
    password: String,
    is_active: bool,
    is_admin: bool,
    is_deleted: bool,
}

impl User {
    pub fn new(username: String, email: String, password: String) -> User {
        User {
            username,
            email,
            password,
            is_active: true,
            is_admin: false,
            is_deleted: false,
        }
    }

    pub fn deactivate(&mut self) {
        self.is_active = false;
    }

    pub fn activate(&mut self) {
        self.is_active = true;
    }

    pub fn promote_to_admin(&mut self) {
        self.is_admin = true;
    }

    pub fn demote_from_admin(&mut self) {
        self.is_admin = false;
    }

    pub fn delete(&mut self) {
        self.is_deleted = true;
    }
}
