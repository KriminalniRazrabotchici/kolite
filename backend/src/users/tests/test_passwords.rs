#[cfg(test)]
mod test_passwords {
    use crate::users::hasher::Hasher;


    #[test]
    fn test_password_hashing() {
        let hasher = Hasher::new();
        let password = "password";
        let hashed_password = hasher.hash_password(password);
        let hashed_password: String = hashed_password.into_iter()
            .enumerate()
            .filter(|(index, _)| {
                if index % 2 == 0 {
                    true
                }else {
                    false
                }
            })
            .map(|(_, element)| element.to_string())
            .collect();

        println!("{:#?}", hashed_password);
            

        assert_ne!(password, hashed_password);
    }

    #[test]
    fn test_password_hashing_and_checking() {
        let hasher = Hasher::new();
        let password = "password";
        let hashed_password = hasher.hash_password(password);

        let is_password_correct = hasher.check_password(password, hashed_password);

        assert_eq!(is_password_correct, true);
    }

    #[test]
    fn test_passoword_is_not_correct() {
        let hasher = Hasher::new();
        let actual_pass = "testhahanowaybro";
        let pass = "not_actual_P@ssw0rd";

        let hashed_password = hasher.hash_password(actual_pass);

        let is_password_correct = hasher.check_password(pass, hashed_password);

        assert_eq!(is_password_correct, false)
    }
}
