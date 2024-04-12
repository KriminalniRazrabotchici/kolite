use argon2::Argon2;
use rand::{self, RngCore};

pub struct Hasher;

impl Hasher {
    pub fn new() -> Self {
        Hasher
    }

    pub fn hash_password(&self, password: &str) -> [u8; 128] {
        let pwd_bytes = password.as_bytes();

        let salt = self.generate_salt();
        let hashed_pass = self.get_hash(&pwd_bytes, &salt);

        self.mix_salt_and_pass(&hashed_pass, &salt)
    }

    pub fn check_password(&self, password: &str, hashed_password: [u8; 128]) -> bool {
        let salt = self.extract_salt(&hashed_password); 
        let pwd_bytes = password.as_bytes();

        let pass_transformed = self.get_hash(pwd_bytes, &salt);
        let mixed_pass = self.mix_salt_and_pass(&pass_transformed, &salt);

        self.compare_passwords(&mixed_pass, &hashed_password)
    }

    fn mix_salt_and_pass(&self, password: &[u8; 64], salt: &[u8; 64]) -> [u8; 128] {
        let array_len = password.len();
        let mut output = [0u8; 128];
        for i in 0..array_len {
            let big_i = i * 2;

            output[big_i] = password[i];
            output[big_i + 1] = salt[i];
        }

        output
    }

    fn compare_passwords(&self, password_1: &[u8; 128], password_2: &[u8; 128]) -> bool {
        for i in 0..password_1.len() {
            if password_1[i] != password_2[i] {
                return false;
            }
        } 

        true
    }

    fn get_hash(&self, password: &[u8], salt: &[u8; 64]) -> [u8; 64] {
        let hasher = Argon2::default();
        let mut hashed_pass = [0u8; 64];
        hasher.hash_password_into(password, salt, &mut hashed_pass).unwrap();
        
        hashed_pass
    }

    fn generate_salt(&self) -> [u8; 64] {
        let mut salt = [0u8; 64];
        
        let mut rng = rand::thread_rng();
        rng.fill_bytes(&mut salt);

        salt
        
    }

    fn extract_salt(&self, hashed_password: &[u8; 128]) -> [u8; 64] {
        let mut output = [0u8; 64];

        // Every other number, starting from 1
        // That is how we save the hashed password
        // The pattern is [pass, salt, pass, salt, ...]
        let mut password_index = 1;

        for i in 0..output.len() {
            output[i] = hashed_password[password_index];

            password_index += 2;
        }

        output
    }
}
