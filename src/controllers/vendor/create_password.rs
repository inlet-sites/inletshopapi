use actix_web::{HttpResponse, put, web};
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
pub struct Body {
    pub password: String,
    pub confirm_password: String
}

#[put("/vendor/{vendor_id}/password/{token}")]
pub async fn route(
    db: web::Data<Database>,
    path: web::Path<(String, String)>,
    body: web::Json<Body>
) -> Result<HttpResponse, AppError> {
    //Gather data
    let (vendor_id, token) = path.into_inner();
    let v_id = ObjectId::parse_str(vendor_id)
        .map_err(|_| AppError::InternalError)?;
    let vendor = Vendor::find_by_id(&db, v_id).await?;

    //Logic
    let update_data = handle_create_password(&vendor, body.into_inner(), token)?;

    //Update and respond
    vendor.update(&db, update_data).await?;
    Ok(HttpResponse::Ok().json(json!({"success": true})))
}

pub fn handle_create_password(
    vendor: &Vendor,
    input: Body,
    token: String
) -> Result<Document, AppError> {
    if vendor.pass_hash.is_some() {
        return Err(AppError::forbidden("Vendor password already created"));
    }

    valid_token(&vendor, &token)?;
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
            password: "password123".to_string(),
            confirm_password: "password124".to_string()
        };

        let result = handle_create_password(&v, i, t);
        assert!(matches!(result, Err(AppError::InvalidInput(_))));
    }

    #[test]
    fn rejects_short_password() {
        let t = Uuid::new_v4().to_string();
        let v = create_vendor(false, Some(t.clone()));
        let i = Body {
            password: String::from("password"),
            confirm_password: String::from("password")
        };

        let result = handle_create_password(&v, i, t);
        assert!(matches!(result, Err(AppError::InvalidInput(_))));
    }

    #[test]
    fn rejects_invalid_token() {
        let v = create_vendor(false, None);
        let i = Body {
            password: String::from("password123"),
            confirm_password: String::from("password123")
        };

        let result = handle_create_password(&v, i, String::from("abc"));
        assert!(matches!(result, Err(AppError::Auth)));
    }

    #[test]
    fn rejects_password_exists() {
        let t = Uuid::new_v4().to_string();
        let v = create_vendor(true, Some(t.clone()));
        let i = Body {
            password: String::from("password123"),
            confirm_password: String::from("password123")
        };

        let result = handle_create_password(&v, i, t);
        assert!(matches!(result, Err(AppError::Forbidden(_))));
    }

    #[test]
    fn returns_valid_document() {
        let t = Uuid::new_v4().to_string();
        let p = String::from("password123");
        let v = create_vendor(false, Some(t.clone()));
        let i = Body {
            password: p.clone(),
            confirm_password: p.clone()
        };

        let result = handle_create_password(&v, i, t.clone()).unwrap();
        assert!(result.contains_key("token"));
        assert!(result.contains_key("pass_hash"));
        assert_ne!(result.get("token").unwrap().to_string(), t);
        assert_ne!(result.get("pass_hash").unwrap().to_string(), p);
    }   
}
