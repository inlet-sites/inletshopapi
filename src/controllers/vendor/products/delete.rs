use actix_web::{HttpResponse, HttpRequest, web, delete};
use mongodb::{
    Database,
    bson::{doc, oid::ObjectId}
};
use crate::{
    app_error::AppError,
    auth::vendor_auth,
    models::product::Product
};

#[delete("/vendor/products/{product_id}")]
pub async fn route(
    db: web::Data<Database>,
    path: web::Path<String>,
    req: HttpRequest
) -> Result<HttpResponse, AppError> {
    let vendor = vendor_auth(&db, &req).await?;
    let product_id = ObjectId::parse_str(path.into_inner())
        .map_err(|_| AppError::invalid_input("Invalid product ID"))?;
    Product::delete(&db, product_id, vendor._id).await?;
    Ok(HttpResponse::Ok().json(doc!{"success": true}))
}
