use actix_web::{HttpResponse, web, put};

#[put("/vendor")]
pub async fn route(
    db: web::Data<Database>
) -> Result<HttpResponse, AppError> {
    Ok(HttpResponse::Ok().body("something"))
}
