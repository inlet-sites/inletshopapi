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
#[cfg(test)]
use mongodb::bson::Bson;

#[derive(Deserialize)]
struct Body {
    current_password: String,
    new_password: String,
    confirm_password: String
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

#[cfg(test)]
mod tests {
    use super::*;

    //update_document
    #[test]
    fn creates_valid_document() {
        let result = update_document(String::from("some_hash"));

        assert!(result.contains_key("pass_hash"));
    }
}
