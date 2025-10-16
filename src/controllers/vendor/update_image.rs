use actix_web::{HttpResponse, HttpRequest, web, put};
use actix_multipart::Multipart;
use mongodb::{
    Database,
    bson::doc
};
use libvips::ops;
use std::{fs, io::Write};
use uuid::Uuid;
use futures_util::TryStreamExt;
use crate::{
    app_error::AppError,
    auth::vendor_auth
};

#[put("/vendor/image")]
pub async fn route(
    db: web::Data<Database>,
    payload: Multipart,
    req: HttpRequest
) -> Result<HttpResponse, AppError> {
    let vendor = vendor_auth(&db, &req).await?;

    let mut image_bytes = retrieve_image_as_bytes(payload).await?;
    image_bytes = shrink_and_convert_image(&image_bytes)?;

    if let Some(ref image_str) = vendor.public_data.image {
        remove_file(format!("/home/leemorgan/documents/inletshop/uploads/{}", image_str))?;
    }
    let file_name = save_file(image_bytes)?;

    vendor.update(&db, doc!{"public_data": {"image": &file_name}}).await?;
    
    Ok(HttpResponse::Ok().json(doc!{"image": file_name}))
}

async fn retrieve_image_as_bytes(mut payload: Multipart) -> Result<Vec<u8>, AppError> {
    let mut image_bytes = Vec::new();

    if let Some(mut field) = payload.try_next().await? {
        if field.name() != Some("image") {
            return Err(AppError::invalid_input("Must contain field 'image'"));
        }

        while let Some(chunk) = field.try_next().await? {
            image_bytes.extend_from_slice(&chunk);
        }
    } else {
        return Err(AppError::invalid_input("Must contain field 'image'"));
    }

    Ok(image_bytes)
}

fn shrink_and_convert_image(img: &Vec<u8>) -> Result<Vec<u8>, AppError> {
    let image_bytes = ops::thumbnail_buffer(&img, 500)
        .map_err(|_| AppError::InternalError)?;

    let webp_bytes = ops::webpsave_buffer_with_opts(
        &image_bytes,
        &ops::WebpsaveBufferOptions {
            q: 80,
            effort: 6,
            ..Default::default()
        }
    )
        .map_err(|_| AppError::InternalError)?;

    Ok(webp_bytes)
}

fn remove_file(file_str: String) -> Result<(), AppError> {
    match fs::remove_file(file_str) {
        Ok(_) => Ok(()),
        Err(_) => Err(AppError::InternalError)
    }
}

fn save_file(img: Vec<u8>) -> Result<String, AppError> {
    let file_id = Uuid::new_v4().to_string();
    let file_name = format!("{}.webp", file_id);
    let mut file = fs::File::create(format!("/home/leemorgan/documents/inletshop/uploads/{}", &file_name))
        .map_err(|_| AppError::InternalError)?;
    file.write_all(&img)
        .map_err(|_| AppError::InternalError)?;

    Ok(file_name)
}
