use actix_web::{HttpResponse, HttpRequest, web, delete};
use mongodb::{Database, bson::{doc, oid::ObjectId}};
use serde::Deserialize;
use crate::{
    app_error::AppError,
    auth::vendor_auth,
    models::product::Price
};

#[derive(Deserialize)]
struct Parameters {
    product_id: String,
    price_id: String
}

#[delete("/vendor/products/{product_id}/prices/{price_id}")]
pub async fn route(
    db: web::Data<Database>,
    path: web::Path<Parameters>,
    req: HttpRequest
) -> Result<HttpResponse, AppError> {
    let vendor = vendor_auth(&db, &req).await?;

    let product_id = ObjectId::parse_str(&path.product_id)
        .map_err(|_| AppError::invalid_input("Invalid product ID"))?;
    let price_id = ObjectId::parse_str(&path.price_id)
        .map_err(|_| AppError::invalid_input("Invalide price ID"))?;

    Price::delete(&db, product_id, price_id, Some(vendor._id)).await?;

    Ok(HttpResponse::Ok().json(doc!{"success": true}))
}
