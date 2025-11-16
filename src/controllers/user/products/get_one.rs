use actix_web::{HttpResponse, web, get};
use mongodb::{Database, bson::oid::ObjectId};
use crate::{
    app_error::AppError,
    models::product::Product,
    dto::product::{ProductDb, ProductResponse}
};

#[get("/user/products/{product_id}")]
pub async fn route(
    db: web::Data<Database>,
    path: web::Path<String>
) -> Result<HttpResponse, AppError> {
    let id = ObjectId::parse_str(path.into_inner())
        .map_err(|_| AppError::invalid_input("Invalid product ID"))?;
    
    let product: ProductDb = Product::find_by_id(&db, id, ProductDb::projection()).await?;
    let response: ProductResponse = product.into();

    Ok(HttpResponse::Ok().json(response))
}

