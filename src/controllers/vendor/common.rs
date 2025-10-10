use argon2::{
    Argon2,
    password_hash::{
        PasswordHash,
        SaltString,
        PasswordVerifier,
        PasswordHasher,
        rand_core::OsRng
    }
};
use crate::app_error::AppError;

pub fn compare_password(password: &String, hash: &String) -> Result<(), AppError> {
    let parsed_hash = PasswordHash::new(hash)
        .map_err(|_| AppError::InternalError)?;

    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .map_err(|_| AppError::Auth)
}

pub fn valid_password(pass: &String, confirm_pass: &String) -> Result<(), AppError> {
    if *pass != *confirm_pass {
        return Err(AppError::invalid_input("Passwords do not match"));
    }

    if pass.chars().count() < 10 {
        return Err(AppError::invalid_input("Password must contain at least 10 characters"));
    }

    Ok(())
}

pub fn hash_password(pass: &String) -> Result<String, AppError> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    match argon2.hash_password(pass.as_bytes(), &salt) {
        Ok(h) => Ok(h.to_string()),
        Err(_) => Err(AppError::InternalError)
    }
}
