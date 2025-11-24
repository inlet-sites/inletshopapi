use actix_web::{HttpResponse, HttpRequest, web, post};
use actix_multipart::form::{MultipartForm, tempfile::TempFile, text::Text};
use serde::Deserialize;
use mongodb::{Database, bson::{Document, doc, oid::ObjectId}};
use uuid::Uuid;
use futures::future::join_all;
use crate::{
    app_error::AppError,
    auth::vendor_auth,
    helpers::{shrink_and_write_image, delete_files},
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

    process_files_thread(
        body,
        vendor._id,
        product_id,
        db.get_ref().clone()
    );

    Ok(HttpResponse::Accepted().json(doc!{"success": true}))
}

fn process_files_thread(body: Body, vendor: ObjectId, product: ObjectId, db: Database) {
    tokio::spawn(async move {

        let home = std::env::var("HOME_DIR").expect("HOME_DIR not set");
        let mut handles = Vec::new();

        for (image, id) in body.images.into_iter().zip(body.ids.into_iter()) {
            let (temp_filename, base_dir, url) = build_image_paths(
                id.into_inner(),
                image,
                &home,
                &vendor,
                &product
            );

            let temp_filename_for_task = temp_filename.clone();
            handles.push(tokio::task::spawn_blocking(move || {
                shrink_and_write_image(
                    temp_filename_for_task,
                    String::from("50"),
                    String::from("1000"),
                    base_dir,
                    url
                )
            }));

            delete_files(vec![temp_filename]);
        }

        let urls = gather_succeeded_urls(handles).await;

        if !urls.is_empty() {
            let thumbnail_url = create_thumbnail_url(
                body.thumbnail,
                &home,
                vendor.to_string(),
                product.to_string()
            );
            match Product::update(
                &db,
                product,
                Some(vendor),
                create_update_doc(urls.clone(), thumbnail_url)
            ).await {
                Ok(_) => (),
                Err(_) => {
                    for u in urls {
                        let full_path = format!("/srv{}", u);
                        delete_files(vec![full_path]);
                    }
                }
            };
        }
    });
}

fn build_image_paths(
    id: String,
    image: TempFile,
    home: &String,
    vendor: &ObjectId,
    product: &ObjectId
) -> (String, String, String) {
    let temp_filename = format!("/tmp/{}.upload", Uuid::new_v4());
    image.file.persist(&temp_filename).expect("Failed to persist uploaded file");

    let base_dir = format!("{}srv", home);
    let url = format!(
        "/vendor-{}/product-{}/{}.avif",
        vendor.to_string(),
        product.to_string(),
        id
    );

    let full_path = format!("{}{}", &base_dir, &url);
    let path_obj = std::path::Path::new(&full_path);
    if let Some(parent) = path_obj.parent() {
        std::fs::create_dir_all(parent).expect("Failed to create directory tree");
    }

    (temp_filename, base_dir, url)
}

async fn gather_succeeded_urls(handles: Vec<tokio::task::JoinHandle<Result<String, ()>>>) -> Vec<String> {
    let results = join_all(handles).await;
    let mut urls = Vec::new();
    for r in results {
        if let Ok(Ok(url)) = r {
            urls.push(url);
        }
    }

    urls
}

fn create_thumbnail_url(
    uuid: Option<Text<String>>,
    home: &String,
    vendor_id: String,
    product_id: String
) -> Option<String> {
    match uuid {
        Some(u) => Some(format!(
            "{}srv/vendor-{}/product-{}/{}.avif",
            home,
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
