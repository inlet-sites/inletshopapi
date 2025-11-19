use actix_web::{HttpResponse, HttpRequest, web, get};
use mongodb::{Database, bson::oid::ObjectId};
use crate::{
    app_error::AppError,
    auth::vendor_auth,
    models::product::Product,
    dto::product::{ProductVendorDb, ProductVendorResponse}
};

#[get("/vendor/products/{product_id}")]
pub async fn route(
    db: web::Data<Database>,
    path: web::Path<String>,
    req: HttpRequest
) -> Result<HttpResponse, AppError> {
    let vendor = vendor_auth(&db, &req).await?;
    let product_id = ObjectId::parse_str(path.into_inner())
        .map_err(|_| AppError::invalid_input("Invalid product ID"))?;
    let product: ProductVendorResponse = Product::find_by_id::<ProductVendorDb>(
        &db,
        product_id,
        Some(vendor._id),
        ProductVendorDb::projection()
    ).await?.into();
    Ok(HttpResponse::Ok().json(product))
}
