use actix_web::{HttpResponse, HttpRequest, web, get};
use serde::Deserialize;
use mongodb::Database;
use crate::{
    app_error::AppError,
    auth::vendor_auth,
    models::product::Product,
    helpers::results_per_page,
    dto::product::{ProductShortDb, ProductShortResponse}
};

#[derive(Deserialize)]
struct Parameters {
    page: Option<u64>,
    results: Option<u64>
}

#[get("/vendor/products")]
pub async fn route(
    db: web::Data<Database>,
    query: web::Query<Parameters>,
    req: HttpRequest
) -> Result<HttpResponse, AppError> {
    let vendor = vendor_auth(&db, &req).await?;

    let results_range: (u64, u64) = (10, 100);
    let result = Product::find_by_vendor::<ProductShortDb>(
        &db,
        vendor._id,
        ProductShortDb::projection(),
        query.page.unwrap_or(0),
        results_per_page(results_range.0, results_range.1, query.results.unwrap_or(50))
    )
        .await?
        .into_iter()
        .map(ProductShortResponse::from)
        .collect();

    Ok(HttpResponse::Ok().json(products))
}
