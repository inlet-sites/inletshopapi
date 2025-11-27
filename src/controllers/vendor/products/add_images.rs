use actix_web::{HttpResponse, HttpRequest, web, post};
use actix_multipart::form::{MultipartForm, tempfile::TempFile, text::Text};
use serde::Deserialize;
use mongodb::{Database, bson::{Document, doc, oid::ObjectId}};
use crate::{
    app_error::AppError,
    auth::vendor_auth,
    helpers::{delete_files, process_images},
    models::product::Product
};

#[derive(MultipartForm)]
struct Body {
    #[multipart(limit = "50MB")]
    #[multipart(rename = "images")]
    images: Vec<TempFile>,
    #[multipart(rename = "id")]
    ids: Vec<Text<String>>,
    thumbnail: Option<Text<String>>
}

#[derive(Deserialize)]
struct Parameters {
    product_id: String
}

#[post("/vendor/products/{product_id}/images")]
pub async fn route(
    db: web::Data<Database>,
    path: web::Path<Parameters>,
    MultipartForm(body): MultipartForm<Body>,
    req: HttpRequest
) -> Result<HttpResponse, AppError> {
    let vendor = vendor_auth(&db, &req).await?;
    let product_id = ObjectId::parse_str(path.into_inner().product_id)
        .map_err(|_| AppError::invalid_input("Invalid product id"))?;
    Product::verify_ownership(&db, product_id, vendor._id).await?;

    tokio::spawn(async move {
        let home = std::env::var("HOME_DIR").expect("HOME_DIR not set");

        let image_urls = process_images(
            body.images,
            body.ids.into_iter().map(|i| i.into_inner()).collect(),
            vendor._id,
            product_id,
            &home
        ).await;
        let thumbnail_url = create_thumbnail_url(
            body.thumbnail,
            vendor._id.to_string(),
            product_id.to_string()
        );
        match Product::update(
            &db,
            product_id,
            Some(vendor._id),
            create_update_doc(image_urls.clone(), thumbnail_url)
        ).await {
            Ok(_) => (),
            Err(_) => delete_files(image_urls.into_iter().map(|u| format!("/srv{}", u)).collect(), true)
        }
    });

    Ok(HttpResponse::Accepted().json(doc!{"success": true}))
}

fn create_thumbnail_url(
    uuid: Option<Text<String>>,
    vendor_id: String,
    product_id: String
) -> Option<String> {
    match uuid {
        Some(u) => Some(format!(
            "/vendor-{}/product-{}/{}.avif",
            vendor_id,
            product_id,
            u.into_inner()
        )),
        None => None
    }
}

fn create_update_doc(urls: Vec<String>, thumbnail: Option<String>) -> Document {
    match thumbnail {
        Some(t) => doc!{
            "$push": {"images": {"$each": urls}},
            "$set": {"thumbnail": t}
        },
        None => doc!{"$push": {"images": {"$each": urls}}}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_doc() {
        let urls = vec![String::from("route/to/image")];
        let thumbnail = "/vendor/product/1234.avif".to_string();
        let expected: Document = doc! {
            "$push": {"images": {"$each": urls.clone()}},
            "$set": {"thumbnail": "/vendor/product/1234.avif"}
        };
        let result = create_update_doc(urls, Some(thumbnail));

        assert_eq!(result, expected);
    }

    #[test]
    fn valid_doc_no_thumbnail() {
        let urls = vec!["route/to/image".to_string(), "another/route".to_string()];
        let expected = doc!{"$push": {"images": {"$each": urls.clone()}}};
        let result = create_update_doc(urls, None);

        assert_eq!(result, expected);
    }
}
