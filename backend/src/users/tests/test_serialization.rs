#[cfg(test)]
mod tests {
    use crate::users::model::User;

    #[test]
    fn test_serialization() { 
        let username = "test_user";
        let password = "password";
        let email = "test@email.com";

        let user = User::new(username, password, email);

        let serialized = serde_json::to_string(&user).unwrap();

        let deserialized: User = serde_json::from_str(&serialized).unwrap();

        assert_eq!(user.get_name(), deserialized.get_name());
        assert_eq!(user.get_email(), deserialized.get_email());
        assert_eq!(user.get_password(), deserialized.get_password());
    }
}
