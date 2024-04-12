#[cfg(test)]
mod test_user_model {
    use crate::users::model::User;

    #[test]
    fn test_creating_user() {
        let username = "test_user";
        let password = "password";
        let email = "asd@asd.com";

        let user = User::new(username, password, email);

        let hashed_password = user.get_password();

        assert_eq!(user.get_name(), username);
        assert_eq!(user.get_email(), email);
        assert_eq!(hashed_password.len(), 128);
        assert_ne!(hashed_password, password.as_bytes());
    }
}
