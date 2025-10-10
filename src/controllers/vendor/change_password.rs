use actix_web::{HttpResponse, HttpRequest, web, put};
use mongodb::{
    Database,
    bson::{
        Document,
        doc
    }
};
use serde::Deserialize;
use serde_json::json;
use crate::{
    controllers::vendor::common,
    app_error::AppError,
    auth::vendor_auth
};

#[derive(Deserialize)]
pub struct Body {
    pub current_password: String,
    pub new_password: String,
    pub confirm_password: String
}

#[put("/vendor/password")]
pub async fn route(
    db: web::Data<Database>,
    body: web::Json<Body>,
    req: HttpRequest
) -> Result<HttpResponse, AppError> {
    //Gather data
    let vendor = vendor_auth(&db, &req).await?;

    //Logic
    common::compare_password(&body.current_password, &vendor.pass_hash.as_ref().unwrap())?;
    common::valid_password(&body.new_password, &body.confirm_password)?;
    let pass_hash = common::hash_password(&body.new_password)?;
    let update_doc = update_document(pass_hash);

    //Update Vendor
    vendor.update(&db, update_doc).await?;
    Ok(HttpResponse::Ok().json(json!({"success": true})))
}

fn update_document(pass_hash: String) -> Document {
    doc!{
        "pass_hash": pass_hash
    }
}
