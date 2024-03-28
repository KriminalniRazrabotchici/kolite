use uuid::Uuid;
use hex;
use crate::users::hasher::Hasher;
use serde::{
    de::{self, Deserialize, Deserializer, Visitor}, ser::{Serialize, SerializeStruct, Serializer}
};

#[derive(Debug)]
pub struct User {
    uuid: String,
    name: String,
    email: String,
    password: [u8; 128],
    is_admin: bool,
    is_active: bool,
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

    fn generate_uuid() -> String{ 
        let uuid = Uuid::now_v7()
            .as_bytes()
            .to_owned();

        hex::encode(uuid)
    }

    pub fn get_uuid(&self) -> &str {
        &self.uuid
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_email(&self) -> &str {
        &self.email
    }

    pub fn get_password(&self) -> &[u8; 128] {
        &self.password
    }

    pub fn is_admin(&self) -> bool {
        self.is_admin
    }

    pub fn is_active(&self) -> bool {
        self.is_active
    }

    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }

    pub fn set_email(&mut self, email: &str) {
        self.email = email.to_string();
    }


    pub fn promote(&mut self) {
        self.is_admin = true;
    }
    
    pub fn demote(&mut self) {
        self.is_admin = false;
    }

    pub fn activate(&mut self) {
        self.is_active = true;
    }

    pub fn deactivate(&mut self) {
        self.is_active = false;
    }

}

impl Serialize for User {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer
    {
        let password = self.password.iter()
            .map(|byte| format!("{:02x}", byte))
            .collect::<String>();

        let mut state = serializer.serialize_struct("User", 6)?;
        state.serialize_field("uuid", &self.uuid)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("email", &self.email)?;
        state.serialize_field("password", &password)?;
        state.serialize_field("is_admin", &self.is_admin)?;
        state.serialize_field("is_active", &self.is_active)?;
        state.end() 
    } 

}

impl<'de> Deserialize<'de> for User {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de> 
    { 
        enum Field {
            UUID,
            Name,
            Email,
            Password,
            IsAdmin,
            IsActive,
            Skipped,
        } 


        struct UserVisitor;

        impl<'de> Visitor<'de> for UserVisitor {
            type Value = User;  

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("struct User")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
                where
                    A: de::MapAccess<'de>, 
            {
                let mut uuid: Option<String> = None;
                let mut name: Option<String> = None;
                let mut email: Option<String> = None;
                let mut password: Option<[u8; 128]> = None;
                let mut is_admin: Option<bool> = None;
                let mut is_active: Option<bool> = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        Field::UUID => uuid = Some(map.next_value()?),
                        Field::Name => name = Some(map.next_value()?),
                        Field::Email => email = Some(map.next_value()?),
                        Field::Password => {
                            let pass_in: String = map.next_value()?;
                            let pass_bytes = hex::decode(pass_in)
                                .map_err(|_| de::Error::custom("password could not be read"))?;

                            let mut pass = [0u8; 128];

                            for (i, &byte) in pass_bytes.iter().enumerate() {
                                pass[i] = byte;
                            }
                            password = Some(pass);

                        }
                        Field::IsActive => is_active = Some(map.next_value()?),
                        Field::IsAdmin => is_admin = Some(map.next_value()?),
                        Field::Skipped => {let _ = map.next_value::<de::IgnoredAny>()?; },
                    }
                }

                let uuid = uuid.ok_or_else(|| de::Error::missing_field("uuid"))?;
                let name = name.ok_or_else(|| de::Error::missing_field("name"))?;
                let email = email.ok_or_else(|| de::Error::missing_field("email"))?;
                let password = password.ok_or_else(|| de::Error::missing_field("password"))?;
                let is_admin = is_admin.ok_or_else(|| de::Error::missing_field("is_admin"))?;
                let is_active = is_active.ok_or_else(|| de::Error::missing_field("is_active"))?;

                Ok(User {
                    uuid,
                    name,
                    email,
                    password,
                    is_admin,
                    is_active
                })
            }

        }

        impl <'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                where
                    D: Deserializer<'de> 
            {                
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter) 
                    -> std::fmt::Result {
                        formatter.write_str("Expected a valid User!")
                        
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
                        where
                            E: serde::de::Error, 
                    {
                        match value {
                            "uuid" => Ok(Field::UUID),
                            "name" => Ok(Field::Name),
                            "email" => Ok(Field::Email),
                            "password" => Ok(Field::Password),
                            "is_admin" => Ok(Field::IsAdmin),
                            "is_active" => Ok(Field::IsActive),
                            "_id" => Ok(Field::Skipped),
                            _ => Err(de::Error::unknown_field(value, FIELDS)),
                        }  
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }   
        const FIELDS: &'static [&'static str] = &["uuid", "name", "email", "password", "is_admin", "is_active"];
        deserializer.deserialize_struct("User", FIELDS, UserVisitor)
    }
}
