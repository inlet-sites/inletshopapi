use actix_web::{HttpResponse, put, web};
use uuid::Uuid;
use serde::Deserialize;
use serde_json::json;
use mongodb::{
    bson::{oid::ObjectId, Document, doc},
    Database
};
use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHasher, SaltString
    },
    Argon2
};
use crate::{
    models::vendor::Vendor,
    app_error::AppError
};

#[derive(Deserialize)]
pub struct Body {
    pub password: String,
    pub confirm_password: String
}

#[put("/vendor/{vendor_id}/password/{token}")]
pub async fn route(
    db: web::Data<Database>,
    path: web::Path<(String, String)>,
    body: web::Json<Body>
) -> Result<HttpResponse, AppError> {
    //Gather data
    let (vendor_id, token) = path.into_inner();
    let v_id = ObjectId::parse_str(vendor_id)
        .map_err(|_| AppError::InternalError)?;
    let vendor = Vendor::find_by_id(&db, v_id).await?;

    //Logic
    let update_data = handle_create_password(&vendor, body.into_inner(), token)?;

    //Update and respond
    vendor.update(&db, update_data).await?;
    Ok(HttpResponse::Ok().json(json!({"success": true})))
}

pub fn handle_create_password(
    vendor: &Vendor,
    input: Body,
    token: String
) -> Result<Document, AppError> {
    if vendor.pass_hash.is_some() {
        return Err(AppError::forbidden("Vendor password already created"));
    }

    valid_token(&vendor, &token)?;
    valid_password(&input.password, &input.confirm_password)?;
    let pass_hash = Some(hash_password(&input.password)?);
    
    Ok(doc!{
        "pass_hash": pass_hash,
        "token": Uuid::new_v4().to_string()
    })
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
