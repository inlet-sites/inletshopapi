use actix_web::{HttpResponse, web, post};
use serde::Deserialize;
use serde_json::json;
use mongodb::Database;
use reqwest::Client;
use crate::{
    app_error::AppError,
    models::vendor::Vendor,
    emails::reset_password::reset_password
};

#[derive(Deserialize)]
struct Body {
    email: String
}

#[post("/vendor/password/email")]
pub async fn route(
    db: web::Data<Database>,
    body: web::Json<Body>
) -> Result<HttpResponse, AppError> {
    let email = body.into_inner().email;
    let vendor = Vendor::find_by_email(&db, &email).await?;
    send_email(vendor).await?;
    Ok(HttpResponse::Ok().json(json!({"success": true})))
}

async fn send_email(vendor: Vendor) -> Result<(), AppError> {
    let client = Client::new();

    let zepto_token = std::env::var("ZEPTO_TOKEN")
        .map_err(|_| AppError::InternalError)?;

    let body = json!({
        "from": {
            "address": "support@inletsites.dev"
        },
        "to": [{
            "email_address": {
                "address": vendor.email,
                "name": vendor.owner
            }
        }],
        "subject": "Rest Password for Inlet.Shop",
        "htmlbody": reset_password(vendor.owner,  vendor._id.to_string(), vendor.token)
    });

    let response = client
        .post("https://zeptomail.zoho.com/v1.1/email")
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .header("Authorization", zepto_token)
        .json(&body)
        .send()
        .await?;

    if response.status().is_success() {
        Ok(())
    } else {
        Err(AppError::InternalError)
    }
}
