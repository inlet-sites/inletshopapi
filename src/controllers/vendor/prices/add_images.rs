use actix_web::{HttpResponse, HttpRequest, web, post};
use actix_multipart::form::{MultipartForm, tempfile::TempFile};
use mongodb::{Database, bson::oid::ObjectId};
use serde::Deserialize;
use crate::{
    app_error::AppError,
    auth::vendor_auth
};

#[derive(Deserialize)]
struct Parameters {
    product_id: String,
    price_id: String
}

#[derive(MultipartForm)]
struct Body {
    #[multipart(limit = "50MB")]
    #[multipart(rename = "images")]
    _images: Vec<TempFile>
}

#[post("/vendor/products/{product_id}/prices/{price_id}/images")]
pub async fn route(
    db: web::Data<Database>,
    params: web::Path<Parameters>,
    MultipartForm(_body): MultipartForm<Body>,
    req: HttpRequest
) -> Result<HttpResponse, AppError> {
    let _vendor = vendor_auth(&db, &req).await?;

    let _product_id = ObjectId::parse_str(&params.product_id)
        .map_err(|_| AppError::invalid_input("Invalid product ID"))?;
    let _price_id = ObjectId::parse_str(&params.price_id)
        .map_err(|_| AppError::invalid_input("Invalid price ID"))?;

    Ok(HttpResponse::Ok().body("Something"))
}
