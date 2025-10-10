use actix_web::{HttpResponse, put, web};
use uuid::Uuid;
use serde::Deserialize;
use serde_json::json;
use mongodb::{
    bson::{oid::ObjectId, Document, doc},
    Database
};
use crate::{
    controllers::vendor::common,
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
    common::valid_password(&input.password, &input.confirm_password)?;
    let pass_hash = Some(common::hash_password(&input.password)?);
    
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

