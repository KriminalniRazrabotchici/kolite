#[cfg(test)]
mod tests {
    use crate::{auth::{claims::Claims, errors::JWTError, refresh, Authenticator}, db_operations::cruder::Cruder, users::User};
    use database::{auth::DatabaseConnectionData, handler::DatabaseHandler};
    use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
    use base64::prelude::*;
    use jsonwebtoken::errors::{Error, ErrorKind};
    use time::OffsetDateTime;

    fn create_authenticator() -> Authenticator {
        let secret_base64 = BASE64_STANDARD.encode("top_secret");

        Authenticator::new(Algorithm::default(), secret_base64)
    }

    async fn get_db_handler() -> DatabaseHandler {
        let db_conn = DatabaseConnectionData::new("test".to_string(), "localhost".to_string(), 27017, None); 

        DatabaseHandler::new(db_conn).await.unwrap()
    }

    async fn create_cruder() -> Cruder {
        let db_handler = get_db_handler().await;

        Cruder::from(db_handler)
    }

    async fn create_user(test_name: &str) -> User {
        let user = User::new(test_name, "test_password", "test_email");
        let cruder = create_cruder().await;
        cruder.save(&user).await.unwrap();

        user        
    }
        
    #[test]
    fn test_creating_jwt() {
        let user = User::new("test_user", "test_password", "test_email");

        let authenticator = create_authenticator();

        let jwt = authenticator.issue_token(&user);

        assert_eq!(jwt.is_ok(), true);
    }

    #[tokio::test]
    async fn test_verifying_jwt() {
        let user = create_user("test_verifying_jwt").await;
        let authenticator = create_authenticator();

        let jwt = authenticator.issue_token(&user).unwrap();
        let cruder = create_cruder().await;

        let result = authenticator.verify_token(jwt.get_token().to_string(), &cruder).await;

        assert_eq!(result, true);
    }

    #[tokio::test]
    async fn test_verifying_jwt_with_wrong_secret() {
        let user = create_user("test_verifying_jwt_with_wrong_secret").await;

        let authenticator = create_authenticator();

        let jwt = authenticator.issue_token(&user).unwrap();
        let cruder = create_cruder().await;

        let mut wrong_authenticator = create_authenticator();
        let wrong_secret = BASE64_STANDARD.encode("wrong_secret");
        wrong_authenticator.set_secret(wrong_secret);

        let result = wrong_authenticator.verify_token(jwt.get_token().to_string(), &cruder).await;

        assert_eq!(result, false);
    }

    #[tokio::test]
    async fn test_issuing_jwt_using_refresh_token() {
        let user = create_user("test_issuing_jwt_using_refresh_token").await;

        let authenticator = create_authenticator();

        let jwt = authenticator.issue_token(&user).unwrap();
        let cruder = create_cruder().await;

        let refresh_string = format!("{}:{}", jwt.get_token(), jwt.get_refresh_token());
        let result = authenticator.refresh(refresh_string, &cruder).await;

        assert_eq!(result.is_ok(), true);
    }

    #[tokio::test]
    async fn test_issuing_jwt_using_expired_refresh_token() {
        let user = create_user("test_issuing_jwt_using_expired_refresh_token").await;

        let authenticator = create_authenticator();

        let iat = OffsetDateTime::now_utc();
        let exp = iat - time::Duration::days(1);

        let claims = Claims::custom(user.get_uuid(), user.get_email(), iat, exp);
        let refresh_token = refresh::create_refresh_token(&claims).unwrap();
        let secret = BASE64_STANDARD.encode("top_secret");
        let jwt = encode(&Header::default(), &claims, &EncodingKey::from_base64_secret(&secret).unwrap());

        let cruder = create_cruder().await;

        let refresh_string = format!("{}:{}", jwt.unwrap(), refresh_token);
        let result = authenticator.refresh(refresh_string, &cruder).await;

        match result {
            Ok(_) => assert!(false),
            Err(JWTError::NativeError(e)) => assert_eq!(*e.kind(), ErrorKind::ExpiredSignature),
            Err(_) => assert!(false)
        }
    }
}
