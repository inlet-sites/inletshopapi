use actix_web::{HttpResponse, post, web};
use uuid::Uuid;
use serde::Deserialize;
use serde_json::json;
use mongodb::{
    bson::{oid::ObjectId, Document, doc},
    Database
};
use crate::{
    controllers::vendor::common,
    models::vendor::Vendor,
    app_error::AppError
};
#[cfg(test)]
use crate::controllers::vendor::common::create_vendor;

#[derive(Deserialize)]
struct Body {
    id: ObjectId,
    token: String,
    password: String,
    confirm_password: String
}

#[post("/vendor/password")]
pub async fn route(
    db: web::Data<Database>,
    body: web::Json<Body>
) -> Result<HttpResponse, AppError> {
    let vendor = Vendor::find_by_id(&db, body.id).await?;

    let update_data = handle_create_password(&vendor, body.into_inner())?;

    vendor.update(&db, update_data).await?;
    Ok(HttpResponse::Ok().json(json!({"success": true})))
}

fn handle_create_password(
    vendor: &Vendor,
    input: Body
) -> Result<Document, AppError> {
    if vendor.pass_hash.is_some() {
        return Err(AppError::forbidden("Vendor password already created"));
    }

    valid_token(&vendor, &input.token)?;
    common::valid_password(&input.password, &input.confirm_password)?;
    let pass_hash = Some(common::hash_password(&input.password)?);
    
    Ok(doc!{
        "pass_hash": pass_hash,
        "token": Uuid::new_v4().to_string()
    })
}

fn valid_token(vendor: &Vendor, token: &String) -> Result<(), AppError> {
    if vendor.token == *token {
        return Ok(());
    }
    Err(AppError::Auth)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_mismatched_passwords() {
        let t = Uuid::new_v4().to_string();
        let v = create_vendor(false, Some(t.clone()));
        let i = Body {
            id: ObjectId::parse_str("6735f92ee4a3c2b14bd9f8a1").expect("Create ObjectId failed"),
            token: t,
            password: "password123".to_string(),
            confirm_password: "password124".to_string()
        };

        let result = handle_create_password(&v, i);
        assert!(matches!(result, Err(AppError::InvalidInput(_))));
    }

    #[test]
    fn rejects_short_password() {
        let t = Uuid::new_v4().to_string();
        let v = create_vendor(false, Some(t.clone()));
        let i = Body {
            id: ObjectId::parse_str("6735f92ee4a3c2b14bd9f8a1").expect("Create ObjectId failed"),
            token: t,
            password: String::from("password"),
            confirm_password: String::from("password")
        };

        let result = handle_create_password(&v, i);
        assert!(matches!(result, Err(AppError::InvalidInput(_))));
    }

    #[test]
    fn rejects_invalid_token() {
        let v = create_vendor(false, None);
        let i = Body {
            id: ObjectId::parse_str("6735f92ee4a3c2b14bd9f8a1").expect("Create ObjectId failed"),
            token: Uuid::new_v4().to_string(),
            password: String::from("password123"),
            confirm_password: String::from("password123")
        };

        let result = handle_create_password(&v, i);
        assert!(matches!(result, Err(AppError::Auth)));
    }

    #[test]
    fn rejects_password_exists() {
        let t = Uuid::new_v4().to_string();
        let v = create_vendor(true, Some(t.clone()));
        let i = Body {
            id: ObjectId::parse_str("6735f92ee4a3c2b14bd9f8a1").expect("Create ObjectId failed"),
            token: t,
            password: String::from("password123"),
            confirm_password: String::from("password123")
        };

        let result = handle_create_password(&v, i);
        assert!(matches!(result, Err(AppError::Forbidden(_))));
    }

    #[test]
    fn returns_valid_document() {
        let t = Uuid::new_v4().to_string();
        let p = String::from("password123");
        let v = create_vendor(false, Some(t.clone()));
        let i = Body {
            id: ObjectId::parse_str("6735f92ee4a3c2b14bd9f8a1").expect("Create ObjectId failed"),
            token: t.clone(),
            password: p.clone(),
            confirm_password: p.clone()
        };

        let result = handle_create_password(&v, i).unwrap();
        assert!(result.contains_key("token"));
        assert!(result.contains_key("pass_hash"));
        assert_ne!(result.get("token").unwrap().to_string(), t);
        assert_ne!(result.get("pass_hash").unwrap().to_string(), p);
    }   
}
