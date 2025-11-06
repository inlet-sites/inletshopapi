use actix_web::{HttpResponse, HttpRequest, web, post};
use mongodb::Database;
use reqwest::Client;
use serde_json::{Value, json};
use std::collections::HashMap;
use crate::{app_error::AppError, auth::vendor_auth};

#[post("/vendor/connect/session")]
pub async fn route(
    db: web::Data<Database>,
    req: HttpRequest
) -> Result<HttpResponse, AppError> {
    let vendor = vendor_auth(&db, &req).await?;

    if let Some(stripe) = vendor.stripe {
        let client_secret = create_account_session(stripe.account_id).await?;
        Ok(HttpResponse::Ok().json(json!({"client_secret": client_secret})))
    } else {
        Err(AppError::invalid_input("No Stripe account for this vendor"))
    }
}

async fn create_account_session(id: String) -> Result<String, AppError> {
    let stripe_secret = std::env::var("STRIPE_INLETSITES_KEY")
        .map_err(|_| AppError::InternalError)?;

    let mut params = HashMap::new();
    params.insert("account", id.as_str());
    params.insert("components[account_onboarding][enabled]", "true");

    let client = Client::new();
    let response = client
        .post("https://api.stripe.com/v1/account_sessions")
        .bearer_auth(stripe_secret)
        .form(&params)
        .send()
        .await?;

    match response.status().is_success() {
        true => {
            let data: Value = response.json().await?;
            let client_secret = data["client_secret"]
                .as_str()
                .ok_or(AppError::StripeError)?
                .to_string();
            Ok(client_secret)
        },
        false => Err(AppError::StripeError)
    }
}
