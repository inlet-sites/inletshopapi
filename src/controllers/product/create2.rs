use actix_web::{HttpResponse, HttpRequest, web, post};
use actix_multipart::Multipart;
use serde::Deserialize;
use mongodb::Database;
use futures_util::TryStreamExt;
use crate::{
    app_error::AppError,
    auth::vendor_auth
};

#[derive(Deserialize)]
pub struct Body {
    name: String,
    tags: Option<Vec<String>>,
    images: Option<Vec<Vec<u8>>>,
    prices: Vec<Price>
}

#[derive(Deserialize)]
pub struct Price {
    descriptor: String,
    price: i32,
    quantity: i32,
    shipping: i32,
    images: Option<Vec<Vec<u8>>>,
    purchase_option: String,
}

#[derive(Deserialize)]
pub struct TempBody {
    name: String
}

impl TempBody {
    async fn from_multipart(data: Multipart) -> Result<Body, AppError> {
        let mut name = String::from("");

        while let Some(mut field) = data.try_next().await? {
            match field.name().unwrap().as_str() {
                "name" => name = multipart_field_to_string(field).await?,
                _ => ()
            }
        }
        
        Ok(TempBody {
            name: name
        })
    }
}

#[post("/product")]
pub async fn route(
    db: web::Data<Database>,
    data: Multipart,
    req: HttpRequest
) -> Result<HttpResponse, AppError> {
    let vendor = vendor_auth(&db, &req).await?;
    let body = TempBody::from_multipart(data).await?;
    println!("{}", body.name);
    Ok(HttpResponse::Ok().body("done"))
}

async fn multipart_field_to_string(field: String) -> Result<String, AppError> {
    let mut bytes = Vec::new();

    while let Some(chunk) = field.try_next().await? {
        bytes.extend_from_slice(&chunk);
    }

    Ok(String::from_utf8(bytes).unwrap_or_default())
}
