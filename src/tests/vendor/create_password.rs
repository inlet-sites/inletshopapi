use uuid::Uuid;
use crate::{
    controllers::vendor::create_password,
    tests::vendor::common::create_vendor,
    dto::vendor::CreatePasswordInput,
    app_error::AppError
};

#[test]
fn rejects_mismatched_passwords() {
    let t = Uuid::new_v4().to_string();
    let v = create_vendor(false, Some(t.clone()));
    let i = CreatePasswordInput {
        password: "password123".to_string(),
        confirm_password: "password124".to_string()
    };

    let result = create_password::handle_create_password(&v, i, t);
    assert!(matches!(result, Err(AppError::InvalidInput(_))));
}

#[test]
fn rejects_short_password() {
    let t = Uuid::new_v4().to_string();
    let v = create_vendor(false, Some(t.clone()));
    let i = CreatePasswordInput {
        password: String::from("password"),
        confirm_password: String::from("password")
    };

    let result = create_password::handle_create_password(&v, i, t);
    assert!(matches!(result, Err(AppError::InvalidInput(_))));
}

#[test]
fn rejects_invalid_token() {
    let v = create_vendor(false, None);
    let i = CreatePasswordInput {
        password: String::from("password123"),
        confirm_password: String::from("password123")
    };

    let result = create_password::handle_create_password(&v, i, String::from("abc"));
    assert!(matches!(result, Err(AppError::Auth)));
}

#[test]
fn rejects_password_exists() {
    let t = Uuid::new_v4().to_string();
    let v = create_vendor(true, Some(t.clone()));
    let i = CreatePasswordInput {
        password: String::from("password123"),
        confirm_password: String::from("password123")
    };

    let result = create_password::handle_create_password(&v, i, t);
    assert!(matches!(result, Err(AppError::Forbidden(_))));
}

#[test]
fn returns_valid_document() {
    let t = Uuid::new_v4().to_string();
    let p = String::from("password123");
    let v = create_vendor(false, Some(t.clone()));
    let i = CreatePasswordInput {
        password: p.clone(),
        confirm_password: p.clone()
    };

    let result = create_password::handle_create_password(&v, i, t.clone()).unwrap();
    assert!(result.contains_key("token"));
    assert!(result.contains_key("pass_hash"));
    assert_ne!(result.get("token").unwrap().to_string(), t);
    assert_ne!(result.get("pass_hash").unwrap().to_string(), p);
}
