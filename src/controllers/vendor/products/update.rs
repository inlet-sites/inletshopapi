use actix_web::{HttpResponse, HttpRequest, web, put};
use mongodb::{Database, bson::{Document, oid::ObjectId}};
use serde::Deserialize;
use crate::{
    app_error::AppError,
    auth::vendor_auth,
    models::product::Product,
    dto::product::{ProductVendorDb, ProductVendorResponse}
};

#[derive(Deserialize)]
struct Body {
    name: Option<String>,
    tags: Option<Vec<String>>,
    thumbnail: Option<String>
}

#[put("/vendor/products/{product_id}")]
pub async fn route(
    db: web::Data<Database>,
    path: web::Path<String>,
    body: web::Json<Body>,
    req: HttpRequest
) -> Result<HttpResponse, AppError> {
    let vendor = vendor_auth(&db, &req).await?;

    let update_doc = match create_update_doc(body.into_inner()) {
        Some(d) => d,
        None => return Err(AppError::invalid_input("No update data provided"))
    };

    let product_id = ObjectId::parse_str(path.into_inner())
        .map_err(|_| AppError::invalid_input("Invalid product ID"))?;

    Product::update(&db, product_id, Some(vendor._id), update_doc).await?;
    let product: ProductVendorResponse = Product::find_by_id::<ProductVendorDb>(
        &db,
        product_id,
        Some(vendor._id),
        ProductVendorDb::projection()
    ).await?.into();

    Ok(HttpResponse::Ok().json(product))
}

fn create_update_doc(body: Body) -> Option<Document> {
    let mut document = Document::new();
    let mut set_document = Document::new();

    if let Some(n) = body.name {
        set_document.insert("name", n);
    }

    if let Some(t) = body.tags {
        set_document.insert("tags", t);
    }

    if let Some(t) = body.thumbnail {
        set_document.insert("thumbnail", t);
    }

    
    match set_document.is_empty() {
        true => None,
        false => {
            document.insert("$set", set_document);
            Some(document)
        }
    }
}
