use actix_web::{HttpResponse, web, put};
use mongodb::{
    Database,
    bson::oid::ObjectId
};
use serde::Deserialize;
use crate::{
    app_error::AppError
};

#[derive(Deserialize)]
pub struct Body {
    pub current_password: String,
    pub new_password: String,
    pub confirm_password: String
}

#[put("/vendor/{vendor_id}/password")]
pub async fn route(
    db: web::Data<Database>,
    path: web::Path<String>,
    body: web::Json<Body>
) -> Result<HttpResponse, AppError> {
    //Gather data
    let vendor_id = ObjectId::parse_str(path.into_inner())
        .map_err(|_| AppError::InternalError)?;
    //
    //Confirm existing password
    //Validate new password
    //Hash password
    //Create update document
    //
    //Update Vendor
    //Respond
    Ok(HttpResponse::Ok().body("something"))
}
