use mongodb::{Database, bson::oid::ObjectId};
use actix_web::HttpRequest;

use crate::app_error::AppError;
use crate::models::vendor::Vendor;

pub async fn vendor_auth(
    db: &Database,
    req: &HttpRequest
) -> Result<Vendor, AppError> {
    let cookie = req.cookie("vendor").ok_or(AppError::Auth)?;
    let user_id = cookie.value();
    let object_id = ObjectId::parse_str(user_id)
        .map_err(|_| AppError::InternalError)?;
    let vendor_coll = db.collection::<Vendor>("vendors");
    let vendor = Vendor::find_by_id(&vendor_coll, object_id).await?;
    if vendor.pass_hash.is_none() {
        return Err(AppError::forbidden("Vendor password not set"));
    }
    Ok(Vendor::find_by_id(&vendor_coll, object_id).await?)
}
