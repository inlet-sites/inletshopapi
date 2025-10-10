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
#[cfg(test)]
use crate::models::vendor::{
    Vendor,
    PublicData
};
#[cfg(test)]
use mongodb::bson::{DateTime, oid::ObjectId};
#[cfg(test)]
use uuid::Uuid;

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

#[cfg(test)]
pub fn create_vendor(has_pass: bool, token: Option<String>) -> Vendor {
    Vendor {
        _id: ObjectId::new(),
        email: String::from("john.doe@inletsites.dev"),
        owner: String::from("John Doe"),
        store: String::from("Inlet Sites"),
        url: String::from("inlet-sites"),
        pass_hash: if has_pass {
            Some(hash_password(&"password123".to_string()).unwrap())
        } else{
            None
        },
        token: match token {
            Some(t) => t,
            None => Uuid::new_v4().to_string()
        },
        public_data: PublicData{
            phone: None,
            email: None,
            address: None,
            slogan: None,
            description: None,
            image: None,
            hours: None,
            links: None,
            website: None
        },
        html: None,
        active: true,
        new_order_send_email: false,
        stripe: None,
        created_at: DateTime::now()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    //compare_password
    #[test]
    fn rejects_incorrect_password() {
        let v = create_vendor(true, None);

        let result = compare_password(&String::from("password124"), &v.pass_hash.unwrap());
        assert!(matches!(result, Err(AppError::Auth)));
    }

    #[test]
    fn accepts_correct_password() {
        let v = create_vendor(true, None);

        let result = compare_password(&String::from("password123"), &v.pass_hash.unwrap());
        assert!(matches!(result, Ok(())));
    }

    //valid_password
    #[test]
    fn rejects_mismatched_passwords() {
        let p = String::from("password123");
        let cp = String::from("password124");

        let result = valid_password(&p, &cp);
        assert!(matches!(result, Err(AppError::InvalidInput(_))));
    }

    #[test]
    fn rejects_short_password() {
        let p = String::from("password");

        let result = valid_password(&p.clone(), &p.clone());
        assert!(matches!(result, Err(AppError::InvalidInput(_))));
    }

    #[test]
    fn valid_password_succeeds() {
        let p = String::from("password123");

        let result = valid_password(&p.clone(), &p.clone());
        assert!(matches!(result, Ok(())));
    }

    //hash_password
    #[test]
    fn hash_ne_password() {
        let p = String::from("password123");

        let result = hash_password(&p).unwrap();
        assert_ne!(result, p);
    }
}
