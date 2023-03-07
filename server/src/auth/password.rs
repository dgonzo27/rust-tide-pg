use argon2::{self, Config};


pub fn validate_password(hash_string: &str, password: String) -> bool {
    let password_bytes = password.as_bytes();
    let salt = hash_string.as_bytes();
    let config = Config::default();
    let hash = argon2::hash_encoded(password_bytes, salt, &config).unwrap();
    
    argon2::verify_encoded(&hash, password_bytes).unwrap()
}

pub fn hash_password(hash_string: &str, password: String) -> String {
    let password_bytes = password.as_bytes();
    let salt = hash_string.as_bytes();
    let config = Config::default();

    argon2::hash_encoded(password_bytes, salt, &config).unwrap()
}
