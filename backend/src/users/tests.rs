#[cfg(tests)]
mod tests {
    use jsonwebtoken::TokenData;

    fn test_creating_user() {
        let user = User::new("test_user", "test_password", "test_email");

        assert_eq!(user.username, "test_user");
        assert_ne!(user.password, "test_password");
        assert_eq!(user.email, "test_email");

    }
}
