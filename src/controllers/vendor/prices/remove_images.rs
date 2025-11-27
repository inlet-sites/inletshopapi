use actix_web::{HttpResponse, HttpRequest, web, delete};
use serde::Deserialize;
use mongodb::{Database, bson::{doc, oid::ObjectId}};
use crate::{
    app_error::AppError,
    auth::vendor_auth,
    helpers::delete_files,
    models::product::Price
};

#[derive(Deserialize)]
struct Parameters {
    product_id: String,
    price_id: String
}

#[delete("/vendor/products/{product_id}/prices/{price_id}/images")]
pub async fn route(
    db: web::Data<Database>,
    params: web::Path<Parameters>,
    body: web::Json<Vec<String>>,
    req: HttpRequest
) -> Result<HttpResponse, AppError> {
    let vendor = vendor_auth(&db, &req).await?;

    let product_id = ObjectId::parse_str(&params.product_id)
        .map_err(|_| AppError::invalid_input("Invalid product ID"))?;
    let price_id = ObjectId::parse_str(&params.price_id)
        .map_err(|_| AppError::invalid_input("Invalid price ID"))?;

    let image_urls = body.into_inner();

    Price::update(
        &db,
        product_id,
        price_id,
        vendor._id,
        doc!{"$pullAll": {"prices.$.images": &image_urls}}
    ).await?;

    delete_files(image_urls, true);

    Ok(HttpResponse::Ok().json(doc!{"success": true}))
}
