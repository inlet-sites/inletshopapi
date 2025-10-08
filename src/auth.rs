use mongodb::{Database, bson::oid::ObjectId};
use actix_web::HttpRequest;

use crate::app_error::AppError;
use crate::models::vendor::Vendor;

pub async fn user_auth(
    db: &Database,
    req: &HttpRequest
) -> Result<Vendor, AppError> {
    let cookie = req.cookie("vendor").ok_or(AppError::Auth)?;
    let user_id = cookie.value();
    let object_id = ObjectId::parse_str(user_id)
        .map_err(|_| AppError::InternalError)?;
    let user_collection = db.collection::<Vendor>("users");
    Ok(Vendor::find_by_id(&user_collection, object_id).await?)
}
