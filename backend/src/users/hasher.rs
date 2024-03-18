use argon2::Argon2;
use rand::{self, RngCore};

struct Hasher;

impl Hasher {
    fn hash_password(&self, password: &str) -> [u8; 128] {
        let pwd_bin = password.as_bytes();

        let hasher = Argon2::default();
        let mut salt = [0u8; 64];
        
        let rng = rand::thread_rng();

        // Random salt per pass
        rng.fill_bytes(&mut salt);
        let mut hashed_pass = [0u8; 64];
        hasher.hash_password_into(pwd_bin, &salt, &mut hashed_pass);
        
        self.mix_salt_and_pass(&hashed_pass, &salt)
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
}
