use actix_web::{HttpResponse, web, get};
use serde::Deserialize;
use mongodb::{
    Database,
    bson::oid::ObjectId
};
use crate::{
    app_error::AppError,
    models::product::Product,
    helpers::results_per_page::results_per_page,
    dto::product::{ProductShortDb, ProductShortResponse}
};

#[derive(Deserialize)]
struct Parameters {
    page: Option<u64>,
    results: Option<u64>
}

#[get("/user/vendors/{vendor_id}/products")]
pub async fn route(
    db: web::Data<Database>,
    path: web::Path<String>,
    query: web::Query<Parameters>
) -> Result<HttpResponse, AppError> {
    let vendor_id = ObjectId::parse_str(path.into_inner())
        .map_err(|_| AppError::invalid_input("Invalid vendor ID"))?;
    let results_range = (10, 100);
    let products: Vec<ProductShortDb> = Product::find_by_vendor(
        &db,
        vendor_id,
        ProductShortDb::projection(),
        query.page.unwrap_or(0),
        results_per_page(results_range.0, results_range.1, query.results.unwrap_or(50))
    ).await?;
    let response_products: Vec<ProductShortResponse> = products
        .into_iter()
        .map(ProductShortResponse::from)
        .collect();
    Ok(HttpResponse::Ok().json(response_products))
}
