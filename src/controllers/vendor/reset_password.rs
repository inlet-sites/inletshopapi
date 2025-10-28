use actix_web::{HttpResponse, web, post};
use serde::Deserialize;
use uuid::Uuid;
use serde_json::json;
use mongodb::{
    Database,
    bson::{
        Document,
        doc,
        oid::ObjectId
    }
};
use crate::{
    app_error::AppError,
    models::vendor::Vendor,
    controllers::vendor::common
};

#[derive(Deserialize)]
struct Body {
    vendor: ObjectId,
    token: String,
    password: String,
    confirm_password: String
}

#[post("/vendor/password/reset")]
pub async fn route(
    db: web::Data<Database>,
    body: web::Json<Body>
) -> Result<HttpResponse, AppError> {
    let vendor = Vendor::find_by_id(&db, body.vendor).await?;
    token_match(&body.token, &vendor.token)?;
    common::valid_password(&body.password, &body.confirm_password)?;
    let updates = create_update_doc(&body.password)?;
    vendor.update(&db, updates).await?;
    Ok(HttpResponse::Ok().json(json!({"success": true})))
}

fn token_match(b_token: &String, v_token: &String) -> Result<(), AppError> {
    if b_token == v_token {
        Ok(())
    } else {
        Err(AppError::forbidden("Invalid authorization"))
    }
}

fn create_update_doc(pass: &String) -> Result<Document, AppError> {
    Ok(doc!{
        "pass_hash": common::hash_password(pass)?,
        "token": Uuid::new_v4().to_string()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    //token_match
    #[test]
    fn matching_token_succeeds() {
        let token = Uuid::new_v4().to_string();

        let result = token_match(&token, &token).unwrap();
        assert_eq!(result, ());
    }

    #[test]
    fn bad_token_fails() {
        let token_one = Uuid::new_v4().to_string();
        let token_two = Uuid::new_v4().to_string();

        let result = token_match(&token_one, &token_two);
        assert!(result.is_err());
    }

    //create_update_doc
    #[test]
    fn creates_valid_doc() {
        let result = create_update_doc(&String::from("password123")).unwrap();

        assert!(!result.is_empty());
        assert!(result.contains_key("pass_hash"));
        assert!(result.contains_key("token"));
        assert_ne!(result.get_str("pass_hash").unwrap(), "password123");
        assert!(Uuid::parse_str(&result.get_str("token").unwrap()).is_ok());
    }
}
