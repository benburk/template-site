use argon2::password_hash::SaltString;
use argon2::password_hash::rand_core::OsRng;
use argon2::{Argon2, PasswordHasher as _};

pub fn generate_hash(plaintext_password: &str) -> String {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(plaintext_password.as_bytes(), &salt)
        .unwrap();
    password_hash.to_string()
}

pub fn validate_password(password: &str) -> String {
    if password.len() < 8 || password.len() > 60 || !password.is_ascii() {
        String::from(
            "Password must be between 8 and 60 characters and only contain ascii characters.",
        )
    } else {
        String::new()
    }
}

pub fn validate_username(username: &str) -> String {
    if username.len() < 5
        || username.len() > 20
        || !username.chars().all(char::is_alphanumeric)
    {
        String::from(
            "Username must be between 5 and 20 characters, and only contain letters / numbers.",
        )
    } else {
        String::new()
    }
}
