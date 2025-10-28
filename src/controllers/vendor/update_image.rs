use actix_web::{HttpResponse, HttpRequest, web, put};
use actix_multipart::Multipart;
use mongodb::{Database, bson::doc};
use tokio::{
    fs,
    process::Command,
    io::AsyncWriteExt,
};
use serde::Deserialize;
use serde_json::json;
use uuid::Uuid;
use std::{
    collections::HashMap,
    process::Stdio
};
use crate::{
    controllers::vendor::common::read_multipart,
    app_error::AppError,
    auth::vendor_auth
};

#[derive(Deserialize)]
struct Body {
    image: Vec<u8>
}

impl Body {
    fn from_map(mut map: HashMap<String, Vec<u8>>) -> Result<Body, AppError> {
        Ok(Body{
            image: match map.remove("image") {
                Some(v) => v,
                None => return Err(AppError::invalid_input("'image' field must contain an image file"))
            }
        })
    }
}

#[put("/vendor/image")]
pub async fn route(
    db: web::Data<Database>,
    payload: Multipart,
    req: HttpRequest
) -> Result<HttpResponse, AppError> {
    let vendor = vendor_auth(&db, &req).await?;
    let body = Body::from_map(read_multipart(payload).await?)?;
    let image = shrink_image(body.image).await?;
    let id = write_image(image)?;
    vendor.update(&db, doc!{"public_data": doc!{"image": format!("/thumbnails/{}.avif", &id)}}).await?;
    Ok(HttpResponse::Ok().json(json!({"image": format!("/thumbnails/{}.avif", &id)})))
}

async fn shrink_image(image: Vec<u8>) -> Result<Vec<u8>, AppError> {
    let mut child = Command::new("sharp")
        .args([
            "--format", "avif",
            "--quality", "50",
            "resize", "1000"
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .map_err(|_| AppError::InternalError)?;

    if let Some(mut stdin) = child.stdin.take() {
        stdin.write_all(&image).await.map_err(|_| AppError::InternalError)?;
        stdin.shutdown().await.map_err(|_| AppError::InternalError)?;
    }

    let output = child.wait_with_output().await.map_err(|_| AppError::InternalError)?;

    if output.status.success(){
        Ok(output.stdout)
    } else {
        Err(AppError::InternalError)
    }
}

fn write_image(image: Vec<u8>) -> Result<String, AppError> {
    let id = Uuid::new_v4().to_string();
    let id_for_task = id.clone();

    tokio::spawn(async move {
        let dir = "/srv/inletshop/thumbnails";
        //let _ = fs::create_dir_all(dir).await;
        match fs::create_dir_all(dir).await {
            Ok(_) => (),
            Err(e) => println!("{}", e)
        };
        let _ = fs::write(format!("{}/{}.avif", dir, id_for_task), image).await;
    }); 

    Ok(id)
}
