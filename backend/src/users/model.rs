use uuid::Uuid;
use crate::users::hasher::Hasher;


#[derive(Debug)]
pub struct User {
    pub(crate) uuid: [u8; 16],
    pub(crate) name: String,
    pub(crate) email: String,
    pub(crate) password: [u8; 128],
    pub(crate) is_admin: bool,
    pub(crate) is_active: bool,
}

impl User {
    pub fn new(username: &str, password: &str, email: &str) -> User {
        let hasher = Hasher::new();

        let uuid = User::generate_uuid();
        let name = username.to_string();
        let email = email.to_string();

        let password = hasher.hash_password(password);


        User {
            uuid,
            name,
            email,
            password,
            is_admin: false,
            is_active: true
        }
    }

    fn generate_uuid() -> [u8; 16]{ 
        let uuid = Uuid::now_v7()
            .as_bytes()
            .to_owned();

        uuid
    }
}

// TODO implement custom Serializer and Deserializer
