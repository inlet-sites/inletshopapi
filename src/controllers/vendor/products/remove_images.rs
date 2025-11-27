use actix_web::{HttpResponse, HttpRequest, web, delete};
use mongodb::{Database, bson::{doc, oid::ObjectId}};
use serde::Deserialize;
use crate::{
    app_error::AppError,
    auth::vendor_auth,
    models::product::Product,
    helpers::delete_files
};

#[derive(Deserialize)]
struct Parameters {
    product_id: String
}

#[delete("/vendor/products/{product_id}/images")]
pub async fn route(
    db: web::Data<Database>,
    path: web::Path<Parameters>,
    body: web::Json<Vec<String>>,
    req: HttpRequest
) -> Result<HttpResponse, AppError> {
    let vendor = vendor_auth(&db, &req).await?;

    let product_id = ObjectId::parse_str(path.into_inner().product_id)
        .map_err(|_| AppError::invalid_input("Invalid product ID"))?;

    let image_urls = body.into_inner();

    Product::update(
        &db,
        product_id,
        Some(vendor._id),
        doc!{"$pullAll": {"images": &image_urls}}
    ).await?;

    delete_files(image_urls, true);

    Ok(HttpResponse::Ok().json(doc!{"success": true}))
}
