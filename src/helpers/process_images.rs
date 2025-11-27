use actix_multipart::form::tempfile::TempFile;
use crate::helpers::delete_files;
use tokio::task::{JoinHandle, spawn_blocking};
use mongodb::bson::oid::ObjectId;
use uuid::Uuid;
use futures::future::join_all;
use std::process::Command;

pub async fn process_images(
    images: Vec<TempFile>,
    ids: Vec<String>,
    vendor: ObjectId,
    product: ObjectId,
    home: &String
) -> Vec<String> {
    let mut handles = Vec::new();
    let mut temp_files = Vec::new();

    for (image, id) in images.into_iter().zip(ids.into_iter()){
        let (temp_filename, base_dir, url) = build_image_paths(
            id,
            image,
            &home,
            &vendor,
            &product
        );

        let temp_filename_for_task = temp_filename.clone();
        handles.push(spawn_blocking(move || {
            let result = Command::new("sharp")
                .args([
                    "--input", temp_filename_for_task.as_str(),
                    "--format", "avif",
                    "--quality", "50",
                    "resize", "1000",
                    "--output", &format!("{}{}", base_dir, url)
                ])
                .stdout(std::process::Stdio::null())
                .status();

            match result {
                Ok(status) if status.success() => Ok(url),
                _ => Err(())
            }
        }));
        temp_files.push(temp_filename);
    }

    let urls = gather_succeeded_urls(handles).await;
    delete_files(temp_files, false);
    urls
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

async fn gather_succeeded_urls(handles: Vec<JoinHandle<Result<String, ()>>>) -> Vec<String> {
    let results = join_all(handles).await;
    let mut urls = Vec::new();
    for r in results {
        if let Ok(Ok(url)) = r {
            urls.push(url);
        }
    }

    urls
}
