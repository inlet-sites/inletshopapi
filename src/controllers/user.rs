use actix_web::{post, web};
use mongodb::{Database}

#[post("/user/login")]
pub async fn login_route(
    db: web::Data<Database>,
    body: web::Json<LoginInput>
) -> Result<HttpResponse, AppError> {
    println!("peen");
    Ok(HttpResponse::Ok().body("double peen"))
}
