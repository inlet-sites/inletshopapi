use actix_web::{HttpResponse, put, web};
use uuid::Uuid;
use serde_json::json;
use mongodb::{
    bson::{oid::ObjectId, doc},
    Database
};
use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHasher, SaltString
    },
    Argon2
};

use crate::models::vendor::Vendor;
use crate::app_error::AppError;
use crate::dto::vendor::CreatePasswordInput;

#[put("/vendor/{vendor_id}/password/{token}")]
pub async fn create_password_route(
    db: web::Data<Database>,
    path: web::Path<(String, String)>,
    body: web::Json<CreatePasswordInput>
) -> Result<HttpResponse, AppError> {
    let (vendor_id, token) = path.into_inner();
    let v_id = ObjectId::parse_str(vendor_id)
        .map_err(|_| AppError::InternalError)?;
    let vendor_coll = db.collection::<Vendor>("vendors");
    let vendor = Vendor::find_by_id(&vendor_coll, v_id).await?;

    let update_data = handle_create_password(vendor, body)?;

    vendor.update(&vendor_coll, update_data).await?;
    Ok(HttpResponse::Ok().json(json!({"success": true})))
}

fn handle_create_password(
    vendor: Vendor,
    input: CreatePasswordInput,
    token: String
) -> Result<(), AppError> {
    if vendor.pass_hash.is_some() {
        return Err(AppError::forbidden("Vendor password already created"));
    }

    valid_token(&vendor, &token)?;
    valid_password(&body.password, &body.confirm_password)?;
    let pass_hash = Some(hash_password(&body.password)?);
    
    return doc!{
        "pass_hash": pass_hash,
        "token": Uuid::new_v4().to_string()
    };
}

fn valid_token(vendor: &Vendor, token: &String) -> Result<(), AppError> {
    if vendor.token == *token {
        return Ok(());
    }
    Err(AppError::Auth)
}

fn valid_password(pass: &String, confirm_pass: &String) -> Result<(), AppError> {
    if *pass != *confirm_pass {
        return Err(AppError::invalid_input("Passwords do not match"));
    }

    if pass.chars().count() < 10 {
        return Err(AppError::invalid_input("Password must contain at least 10 characters"));
    }

    Ok(())
}

fn hash_password(pass: &String) -> Result<String, AppError> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    match argon2.hash_password(pass.as_bytes(), &salt) {
        Ok(h) => Ok(h.to_string()),
        Err(_) => Err(AppError::InternalError)
    }
}
