use actix_web::{HttpResponse, web, get};
use mongodb::Database;
use serde::Serialize;
use crate::{
    models::vendor::{Vendor, PublicData},
    app_error::AppError
};

#[derive(Serialize)]
struct ResponseVendor {
    id: String,
    store: String,
    url: String,
    public_data: PublicData,
    html: Option<String>
}

#[get("/user/vendors/{vendor_url}")]
pub async fn route(
    db: web::Data<Database>,
    path: web::Path<String>
) -> Result<HttpResponse, AppError> {
    let vendor = Vendor::find_by_url(&db, &path.into_inner()).await?;

    Ok(HttpResponse::Ok().json(response(vendor)))
}

fn response(v: Vendor) -> ResponseVendor {
    ResponseVendor {
        id: v._id.to_string(),
        store: v.store,
        url: v.url,
        public_data: v.public_data,
        html: v.html
    }
}
