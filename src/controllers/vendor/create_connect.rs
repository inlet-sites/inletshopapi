use actix_web::{HttpResponse, HttpRequest, web, post};
use mongodb::{Database, bson::{Document, doc}};
use serde_json::{Value, json};
use std::collections::HashMap;
use reqwest::Client;
use crate::{
    app_error::AppError,
    auth::vendor_auth,
    models::vendor::Vendor
};

#[post("/vendor/connect")]
pub async fn route(
    db: web::Data<Database>,
    req: HttpRequest
) -> Result<HttpResponse, AppError> {
    let vendor = vendor_auth(&db, &req).await?;

    if let Some(stripe_data) = vendor.stripe {
        return Ok(HttpResponse::Ok().json(json!({"account": stripe_data.account_id})));
    }

    let account_number = create_account(&vendor).await?;
    let data = create_update_doc(&account_number);

    vendor.update(&db, data).await?;
    Ok(HttpResponse::Ok().json(json!({"account": account_number})))
}

async fn create_account(vendor: &Vendor) -> Result<String, AppError> {
    let stripe_secret = std::env::var("STRIPE_INLETSITES_KEY")
        .map_err(|_| AppError::InternalError)?;

    let mut params = HashMap::new();
    params.insert("type", "express");
    params.insert("country", "US");
    params.insert("email", &vendor.email);
    params.insert("business_type", "company");
    params.insert("company[name]", &vendor.store);

    let client = Client::new();
    let response = client
        .post("https://api.stripe.com/v1/accounts")
        .bearer_auth(stripe_secret)
        .form(&params)
        .send()
        .await?;

    if response.status().is_success() {
        let data: Value = response.json().await?;
        let id = data["id"]
            .as_str()
            .ok_or(AppError::StripeError)?
            .to_string();
        Ok(id)
    } else {
        Err(AppError::StripeError)
    }
}

fn create_update_doc(account_number: &String) -> Document {
    doc!{
        "stripe": doc!{
            "account_id": account_number,
            "activated": false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    //create_update_doc
    #[test]
    fn creates_valid_doc() {
        let result = create_update_doc(&String::from("123456"));
        let stripe_doc = result.get_document("stripe").expect("Stripe sub-document doesn't exist");

        assert!(!stripe_doc.is_empty());
        assert!(stripe_doc.contains_key("account_id"));
        assert!(stripe_doc.contains_key("activated"));
    }
}
