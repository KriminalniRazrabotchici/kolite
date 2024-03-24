#[cfg(test)]
mod test_user_model {
    use crate::users::model::User;

    #[test]
    fn test_creating_user() {
        let username = "test_user";
        let password = "password";
        let email = "asd@asd.com";

        let user = User::new(username, password, email);

        assert_eq!(user.name, username);
        assert_eq!(user.email, email);
        assert!(user.password.len() == 128);
        assert!(user.is_admin == false);
        assert!(user.is_active == true);
    }
}
