use actix_web::{HttpResponse, HttpRequest, web, post};
use mongodb::Database;
use reqwest::Client;
use crate::{app_error::AppError, auth::vendor_auth};

#[post("/vendor/connect/session")]
pub async fn route(
    db: web::Data<Database>,
    body: web::Json<Body>,
    req: HttpRequest
) -> Result<HttpResponse, AppError> {
    let vendor = vendor_auth(&db, &req).await?;

    let client_secret = create_account_session(vendor.stripe.account_id);

    Ok(HttpResponse::Ok().json())
}
