use actix_web::{HttpResponse, HttpRequest, web, put};
use actix_multipart::Multipart;
use mongodb::Database;
use libvips::{VipsImage, ops};
use futures_util::TryStreamExt;
use std::fs::File;
use std::io::Write;
use crate::{
    app_error::AppError,
    auth::vendor_auth
};

#[put("/vendor/image")]
pub async fn route(
    db: web::Data<Database>,
    mut payload: Multipart,
    req: HttpRequest
) -> Result<HttpResponse, AppError> {
    println!("libvips init? {}", libvips::vips_initialized());
    //Extract multipart field
    let mut field = payload
        .try_next()
        .await?
        .ok_or(AppError::invalid_input("No image field found"))?;

    if field.name() != Some("image") {
        return Err(AppError::invalid_input("No image field found"));
    }

    //Read image bytes into memory
    let mut image_data = Vec::new();
    let mut total_size = 0usize;

    while let Some(chunk) = field.try_next().await? {
        total_size += chunk.len();
        if total_size > 25 * 1024 * 1024 {
            return Err(AppError::invalid_input("Images cannot exceed 25MB"));
        }
        image_data.extend_from_slice(&chunk);
    }

    if image_data.is_empty() {
        return Err(AppError::invalid_input("Empty image upload"));
    }

    println!("Loaded {} bytes", image_data.len());

    //Decode with libvips
    let vips_img = VipsImage::new_from_buffer(&image_data, "")
        .map_err(|_| AppError::InternalError)?;

    //Compute scale to max 1000px
    let width = vips_img.get_width() as f64;
    let height = vips_img.get_height() as f64;
    let max_dim = width.max(height);
    let scale = (1000.0 / max_dim).min(1.0);

    //Resize if necessary
    let resized_img = if scale < 1.0 {
        ops::resize(&vips_img, scale).map_err(|_| AppError::InternalError)?
    } else {
        vips_img
    };

    println!("one");
    //Convert to WebP
    let webp_opts = ops::WebpsaveBufferOptions {
        q: 75,
        ..Default::default()
    };

    let webp_data = ops::webpsave_buffer_with_opts(&resized_img, &webp_opts)
        .map_err(|_| AppError::InternalError)?;


    println!("two");
    //Save to file
    let filename = "./output.webp";
    let mut file = File::create(filename)
        .map_err(|_| AppError::InternalError)?;

    file.write_all(&webp_data)
        .map_err(|_| AppError::InternalError)?;

    println!("Saved compressed webp to {}", filename);
    
    Ok(HttpResponse::Ok().body("Finished"))
}
