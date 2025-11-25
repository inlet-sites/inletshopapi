use actix_web::{HttpResponse, HttpRequest, web, put};
use mongodb::{Database, bson::{Document, doc, oid::ObjectId}};
use serde::Deserialize;
use crate::{
    app_error::AppError,
    auth::vendor_auth,
    models::product::Price,
    dto::price::{VendorDb, VendorResponse}
};

#[derive(Deserialize)]
struct Body {
    descriptor: Option<String>,
    quantity: Option<i32>
}

#[derive(Deserialize)]
struct Parameters {
    product_id: String,
    price_id: String
}

#[put("/vendor/products/{product_id}/prices/{price_id}")]
pub async fn route(
    db: web::Data<Database>,
    path: web::Path<Parameters>,
    body: web::Json<Body>,
    req: HttpRequest
) -> Result<HttpResponse, AppError> {
    let vendor = vendor_auth(&db, &req).await?;

    let product_id = ObjectId::parse_str(&path.product_id)
        .map_err(|_| AppError::invalid_input("Invalid product ID"))?;
    let price_id = ObjectId::parse_str(&path.price_id)
        .map_err(|_| AppError::invalid_input("Invalid price ID"))?;

    let update_doc = create_update_doc(body.into_inner());

    Price::update(&db, product_id, price_id, vendor._id, update_doc).await?;
    let price_response: VendorResponse = Price::find_by_id::<VendorDb>(
        &db,
        product_id,
        price_id,
        Some(vendor._id),
        VendorDb::projection()
    ).await?.into();

    Ok(HttpResponse::Ok().json(price_response))
}

fn create_update_doc(body: Body) -> Document {
    let mut doc = Document::new();
    let mut set_doc = Document::new();

    if let Some(d) = body.descriptor {
        set_doc.insert("descriptor", d);
    }

    if let Some(q) = body.quantity {
        set_doc.insert("quantity", q);
    }

    doc.insert("$set", set_doc);
    doc
}
