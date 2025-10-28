use actix_web::{HttpResponse, web, post};
use serde::Deserialize;
use uuid::Uuid;
use serde_json::json;
use mongodb::{
    Database,
    bson::{
        Document,
        doc,
        oid::ObjectId
    }
};
use crate::{
    app_error::AppError,
    models::vendor::Vendor,
    controllers::vendor::common
};

#[derive(Deserialize)]
struct Body {
    vendor: ObjectId,
    token: String,
    password: String,
    confirm_password: String
}

#[post("/vendor/password/reset")]
pub async fn route(
    db: web::Data<Database>,
    body: web::Json<Body>
) -> Result<HttpResponse, AppError> {
    let vendor = Vendor::find_by_id(&db, body.vendor).await?;
    token_match(&body.token, &vendor.token)?;
    common::valid_password(&body.password, &body.confirm_password)?;
    let updates = create_update_doc(&body.password)?;
    vendor.update(&db, updates).await?;
    Ok(HttpResponse::Ok().json(json!({"success": true})))
}

fn token_match(b_token: &String, v_token: &String) -> Result<(), AppError> {
    if b_token == v_token {
        Ok(())
    } else {
        Err(AppError::forbidden("Invalid authorization"))
    }
}

fn create_update_doc(pass: &String) -> Result<Document, AppError> {
    Ok(doc!{
        "pass_hash": common::hash_password(pass)?,
        "token": Uuid::new_v4().to_string()
    })
}
