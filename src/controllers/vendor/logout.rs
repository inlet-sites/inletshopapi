use actix_web::{HttpResponse, post, cookie::{Cookie, SameSite}};
use mongodb::bson::doc;
use crate::app_error::AppError;

#[post("/vendor/logout")]
pub async fn route() -> Result<HttpResponse, AppError> {
    let mut response = HttpResponse::Ok().json(doc!{"success": true});
    response.add_removal_cookie(&create_removal_cookie())
        .map_err(|_| AppError::InternalError)?;
    Ok(response)
}

fn create_removal_cookie() -> Cookie<'static> {
    if cfg!(debug_assertions){
        Cookie::build("vendor", "")
            .path("/")
            .http_only(true)
            .finish()
    } else {
        Cookie::build("vendor", "")
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

    #[test]
    fn cookie_correct_name() {
        let result = create_removal_cookie();
        assert_eq!(result.name(), "vendor");
    }

    #[test]
    fn cookie_correct_value() {
        let result = create_removal_cookie();
        assert_eq!(result.value(), "");
    }
}
