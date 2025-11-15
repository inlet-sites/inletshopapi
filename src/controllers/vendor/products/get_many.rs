use actix_web::{HttpResponse, HttpRequest, web, get};
use serde::Deserialize;
use mongodb::Database;
use crate::{
    app_error::AppError,
    auth::vendor_auth,
    models::product::Product,
    helpers::results_per_page,
    dto::short_product::ShortProductResponse
};

#[derive(Deserialize)]
struct Parameters {
    page: Option<u64>,
    results: Option<i64>
}

#[get("/vendor/products")]
pub async fn route(
    db: web::Data<Database>,
    query: web::Query<Parameters>,
    req: HttpRequest
) -> Result<HttpResponse, AppError> {
    let vendor = vendor_auth(&db, &req).await?;

    let results_range = (10, 100);
    let products = Product::find_by_vendor(
        &db,
        vendor._id,
        query.page.unwrap_or(0),
        results_per_page(results_range.0, results_range.1, query.results.unwrap_or(50))
    ).await?;

    let response_products: Vec<ShortProductResponse> = products
        .into_iter()
        .map(|p| ShortProductResponse::from_short_product(p)).collect();

    Ok(HttpResponse::Ok().json(response_products))
}
