use actix_web::{HttpResponse, HttpRequest, web, get};
use mongodb::Database;
use crate::{
    app_error::AppError,
    auth::vendor_auth
};

#[get("/vendor")]
pub async fn route(
    db: web::Data<Database>,
    req: HttpRequest
) -> Result<HttpResponse, AppError> {
    let vendor = vendor_auth(&db, &req).await?;
    Ok(HttpResponse::Ok().json(vendor.response()))
}
