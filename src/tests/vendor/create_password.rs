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
