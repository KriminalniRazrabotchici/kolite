use uuid::Uuid;
use argon2::{Argon2, Algorithm};
use rand::{self, RngCore};


#[derive(Debug)]
pub struct User {
    uuid: [u8; 16],
    name: String,
    password: [u8; 64],
    salt: [u8, 64],
    age: i32,
    is_admin: bool,
    is_active: bool,
}

impl User {
    pub fn new(username: &str, password: &str, email: &str) -> User {
        let uuid = generate_uuid();
        User {
        }
    }

    fn generate_uuid() -> String { 
        Uuid::now_v7().to_string() 
    }
}

// TODO implement custom Serializer and Deserializer
