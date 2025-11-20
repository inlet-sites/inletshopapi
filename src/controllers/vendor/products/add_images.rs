use actix_web::{HttpResponse, HttpRequest, web, post};
use actix_multipart::form::{MultipartForm, tempfile::TempFile};
use serde::Deserialize;
use mongodb::{Database, bson::{Document, doc, oid::ObjectId}};
use uuid::Uuid;
use crate::{
    app_error::AppError,
    auth::vendor_auth,
    helpers::shrink_and_write_image,
    models::product::Product
};

#[derive(MultipartForm)]
struct Body {
    #[multipart(limit = "15MB")]
    images: Vec<TempFile>
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
    if body.images.len() > 15 {
        return Err(AppError::invalid_input("Exceeds 10 image maximum."));
    }

    let vendor = vendor_auth(&db, &req).await?;
    let product_id = ObjectId::parse_str(path.into_inner().product_id)
        .map_err(|_| AppError::invalid_input("Invalid product id"))?;
    Product::verify_ownership(&db, product_id, vendor._id).await?;
    let vendor_id = vendor._id;

    tokio::spawn(async move {
        use futures::future::join_all;

        let mut handles = Vec::new();
        let home = std::env::var("HOME_DIR").expect("HOME_DIR not set");

        for image in body.images {
            let temp_filename = format!("/tmp/{}.upload", Uuid::new_v4());
            image.file.persist(&temp_filename).expect("Failed to persist uploaded file");

            let id = Uuid::new_v4().to_string();

            let url = format!(
                "/vendor-{}/product-{}/{}.avif",
                vendor._id.to_string(),
                product_id.to_string(),
                id
            );

            let base_dir = format!("{}srv", home);
            let full_dir = format!("{}{}", &base_dir, &url);
            let path_obj = std::path::Path::new(&full_dir);
            if let Some(parent) = path_obj.parent() {
                std::fs::create_dir_all(parent).expect("Failed to create directory tree");
            }

            let handle = tokio::task::spawn_blocking(move || {
                shrink_and_write_image(
                    temp_filename,
                    String::from("50"),
                    String::from("1000"),
                    base_dir,
                    url
                )
            });

            handles.push(handle);
        }

        let results = join_all(handles).await;
        let mut urls = Vec::new();
        for r in results {
            if let Ok(Ok(url)) = r {
                urls.push(url);
            }
        }

        if !urls.is_empty() {
            match Product::update(&db, product_id, Some(vendor_id), create_update_doc(urls.clone())).await {
                Ok(_) => (),
                Err(_) => {
                    for u in urls {
                        let full_path = format!("/srv{}", u);
                        if let Err(e) = std::fs::remove_file(&full_path) {
                            eprintln!("Failed to remove orphaned file {}: {:?}", full_path, e);
                        }
                    }
                }
            };
        }
    });

    Ok(HttpResponse::Accepted().json(doc!{"success": true}))
}

fn create_update_doc(urls: Vec<String>) -> Document {
    doc!{
        "$push": {
            "images": {"$each": urls}
        }
    }
}
