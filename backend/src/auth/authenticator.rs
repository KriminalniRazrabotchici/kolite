use super::claims::Claims;
use super::refresh::create_refresh_token;
use super::errors::JWTError;

use crate::{db_operations::cruder::Cruder, users::User};
use database::errors::DatabaseError;
use jsonwebtoken::{encode, decode, DecodingKey, EncodingKey, Validation, Header, Algorithm};
use mongodb::bson::doc;


pub struct Authenticator {
    default_header: Header,
    default_validation: Validation, 
    secret: String,
}

impl Authenticator {
    pub fn new(algorithm: Algorithm, secret: String) -> Authenticator {
        let header = Header::new(algorithm);
        let validation = Validation::new(algorithm);

        Authenticator {
            default_header: header,
            default_validation: validation,
            secret,
        }
    } 

    pub(super) fn set_secret(&mut self, secret: String) {
        self.secret = secret;
    }

    pub fn issue_token(&self, user: &User) -> Result<AuthenticationDetails, JWTError>{
        let mut claims = Claims::new(user);

        let refresh_token = create_refresh_token(&mut claims)?;

        let jwt = self.create_jwt(&claims)?;

        let auth_details = AuthenticationDetails {
            token: jwt,
            refresh_token
        };

        Ok(auth_details)
    }

    pub async fn refresh(&self, refresh: String, cruder: &Cruder) -> Result<AuthenticationDetails, JWTError> {
        let [token, refresh_token] = self.split_refresh_string(&refresh)?;

        let decoding_key = DecodingKey::from_base64_secret(&self.secret)?;
        let jwt_result = decode::<Claims>(&token, &decoding_key, &self.default_validation);

        let mut token_data = match jwt_result {
            Ok(token) => token,
            Err(e) => return Err(JWTError::NativeError(e)),
        };   
        
        let check_token = create_refresh_token(&mut token_data.claims)?;

        if check_token == refresh_token {
            let uuid = &token_data.claims.uuid;
            
            match self.check_user(uuid, cruder).await {
                false => return Err(JWTError::custom("This refresh token is invalid")),
                true => {
                    // we know its safe, the check_user function has already checked if the user exists
                    let user = self.get_user(uuid, cruder).await.unwrap();

                    self.issue_token(&user)
                }
           }
        } else {
            Err(JWTError::custom("This refresh token is invalid"))
        }
    }   

    pub async fn verify_token(&self, token: String, cruder: &Cruder) -> bool{
        let decoding_key = DecodingKey::from_base64_secret(&self.secret).unwrap();
        let jwt_result = decode::<Claims>(&token, &decoding_key, &self.default_validation);

        let token_data = if let Ok(data) = jwt_result{
            data
        } else {
            return false;
        };

        if !token_data.claims.is_active {
            return false;
        }

        let uuid = &token_data.claims.uuid;
 
        self.check_user(uuid, cruder).await
    }

    async fn get_user(&self, uuid: &str, cruder: &Cruder) -> Result<User, DatabaseError> {
        let query = doc! {"uuid": uuid};
        let user = cruder.get_one::<User>(Some(query)).await;

        match user {
            Ok(user_option) => {
                match user_option {
                    Some(user) => Ok(user),
                    None => Err(DatabaseError::NotFoundError("User not found".to_string()))
                }
            },
            Err(e) => Err(DatabaseError::CouldNotSaveError(e.to_string()))
        } 
    }

    async fn check_user(&self, uuid: &str, cruder: &Cruder) -> bool {
        let user =  self.get_user(uuid, cruder).await;
        match user {
            Ok(_) => true,
            Err(_) => false
        }
    }

    fn split_refresh_string<'a>(&self, string: &'a str) -> Result<[&'a str; 2], JWTError> {
        let string_iter = string.split(':'); 
        let mut output: Vec<&str> = Vec::new();

        let mut count = 0;
        for value in string_iter {
            if count > 1 {
                return Err(JWTError::custom("Barr AIMMM"));    
            } 

            output.push(value);
            count += 1;
        }

        if output.len() != 2 {
            return Err(JWTError::custom("Barr AIMMM"));    
        }

        Ok([output[0], output[1]])
    }

    fn create_jwt(&self, claims: &Claims) -> Result<String, JWTError> {  
        let encoding_key = EncodingKey::from_base64_secret(&self.secret)?;

        let jwt = encode(&self.default_header, &claims, &encoding_key)?;
        Ok(jwt)
    }
}


pub struct AuthenticationDetails {
    token: String, 
    refresh_token: String
}

impl AuthenticationDetails {
    pub fn get_refresh_token(&self) -> &str {
        &self.refresh_token
    }

    pub fn get_token(&self) -> &str {
        &self.token
    }
}

