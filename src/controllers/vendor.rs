use actix_web::{HttpResponse, post, web};
use mongodb::{Database};

use crate::app_error::AppError;
use crate::dto::vendor::LoginInput;

#[post("/user/login")]
pub async fn login_route(
    db: web::Data<Database>,
    body: web::Json<LoginInput>
) -> Result<HttpResponse, AppError> {
    println!("peen");
    Ok(HttpResponse::Ok().body("double peen"))
}
