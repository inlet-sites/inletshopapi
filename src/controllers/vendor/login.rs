use actix_web::{HttpResponse, post, web, cookie::Cookie};
use serde::Deserialize;
use mongodb::Database;
use crate::{
    controllers::vendor::common,
    models::vendor::Vendor
};

#[derive(Deserialize)]
struct Body {
    email: String,
    password: String
}

#[put("/vendor/login")]
pub async fn route(
    db: web::Data<Database>,
    body: web::Json<Body>
) -> Result<HttpResponse, AppError> {
    //Gather data
    let vendor = Vendor::find_by_email(&db, body.email).await?;

    //Logic
    common::compare_password(&body.password, &vendor.pass_hash)?;
    let cookie = set_auth_cookie(vendor._id);

    //Respond
    Ok(HttpResponse::Ok().cookie(cookie).json(vendor.response()))
}

fn set_auth_cookie()(id: String) -> Cookie<'static> {
    Cookie::build("vendor", id)
        .path("/")
        .http_only(true)
        .finish()
}
