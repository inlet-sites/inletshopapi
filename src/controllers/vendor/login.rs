use actix_web::{HttpResponse, post, web, cookie::{Cookie, SameSite}};
use serde::Deserialize;
use mongodb::Database;
use crate::{
    controllers::vendor::common,
    models::vendor::Vendor,
    app_error::AppError
};

#[derive(Deserialize)]
struct Body {
    email: String,
    password: String
}

#[post("/vendor/login")]
pub async fn route(
    db: web::Data<Database>,
    body: web::Json<Body>
) -> Result<HttpResponse, AppError> {
    //Gather data
    let vendor = Vendor::find_by_email(&db, &body.email).await?;

    //Logic
    common::compare_password(&body.password, &vendor.pass_hash.as_ref().unwrap())?;
    let cookie = set_auth_cookie(vendor._id.to_string());

    //Respond
    Ok(HttpResponse::Ok().cookie(cookie).json(vendor.response()))
}

fn set_auth_cookie(id: String) -> Cookie<'static> {
    if cfg!(debug_assertions){
        Cookie::build("vendor", id)
            .path("/")
            .http_only(true)
            .finish()
    } else{
        Cookie::build("vendor", id)
            .domain(".inletsites.dev")
            .path("/")
            .same_site(SameSite::None)
            .http_only(true)
            .secure(true)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    //set_auth_cookie
    #[test]
    fn cookie_correct_name() {
        let result = set_auth_cookie(String::from("12345"));
        assert_eq!(result.name(), "vendor");
    }

    #[test]
    fn cookie_correct_value() {
        let result = set_auth_cookie(String::from("12345"));
        assert_eq!(result.value(), "12345");
    }
}
