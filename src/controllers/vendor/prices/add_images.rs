use actix_web::{HttpResponse, HttpRequest, web, post};
use actix_multipart::form::{MultipartForm, tempfile::TempFile};
use mongodb::{Database, bson::{Document, doc, oid::ObjectId}};
use serde::Deserialize;
use uuid::Uuid;
use crate::{
    app_error::AppError,
    auth::vendor_auth,
    helpers::{process_images, delete_files},
    models::product::{Product, Price}
};

#[derive(Deserialize)]
struct Parameters {
    product_id: String,
    price_id: String
}

#[derive(MultipartForm)]
struct Body {
    #[multipart(limit = "50MB")]
    #[multipart(rename = "images")]
    images: Vec<TempFile>
}

#[post("/vendor/products/{product_id}/prices/{price_id}/images")]
pub async fn route(
    db: web::Data<Database>,
    params: web::Path<Parameters>,
    MultipartForm(body): MultipartForm<Body>,
    req: HttpRequest
) -> Result<HttpResponse, AppError> {
    let vendor = vendor_auth(&db, &req).await?;

    let product_id = ObjectId::parse_str(&params.product_id)
        .map_err(|_| AppError::invalid_input("Invalid product ID"))?;
    let price_id = ObjectId::parse_str(&params.price_id)
        .map_err(|_| AppError::invalid_input("Invalid price ID"))?;

    Product::verify_ownership(&db, product_id, vendor._id).await?;

    tokio::spawn(async move {
        let home = std::env::var("HOME_DIR").expect("HOME_DIR not set");

        let image_count = body.images.len();
        let image_urls = process_images(
            body.images,
            (0..image_count).map(|_| Uuid::new_v4().to_string()).collect(),
            vendor._id,
            product_id,
            &home
        ).await;
        match Price::update(
            &db,
            product_id,
            price_id,
            vendor._id,
            create_update_doc(image_urls.clone())
        ).await {
            Ok(_) => (),
            Err(_) => delete_files(image_urls.into_iter().map(|u| format!("/srv{}", u)).collect(), true)
        }
    });

    Ok(HttpResponse::Ok().json(doc!{"success": true}))
}

fn create_update_doc(urls: Vec<String>) -> Document {
    doc!{
        "$push": {
            "prices.$.images": {
                "$each": urls
            }
        }
    }
}
