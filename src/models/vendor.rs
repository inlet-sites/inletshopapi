use serde::{Serialize, Deserialize};
use mongodb::{
    bson::{oid::ObjectId, DateTime, doc},
    Collection
};

use crate::app_error::AppError;

#[derive(Serialize, Deserialize)]
pub struct Vendor {
    pub id: ObjectId,
    pub email: String,
    pub owner: String,
    pub store: String,
    pub url: String,
    pub pass_hash: Option<String>,
    pub token: String,
    pub public_data: PublicData,
    pub html: Option<String>,
    pub active: bool,
    pub new_order_send_email: bool,
    pub stripe: Option<StripeData>,
    pub created_at: DateTime
}

#[derive(Serialize, Deserialize)]
pub struct PublicData {
    phone: Option<String>,
    email: Option<String>,
    address: Option<String>,
    slogan: Option<String>,
    description: Option<String>,
    image: Option<String>,
    hours: Option<BusinessHours>,
    links: Option<Vec<Link>>,
    website: Option<String>
}

#[derive(Serialize, Deserialize)]
struct StripeData {
    account_id: String,
    activated: bool
}

#[derive(Serialize, Deserialize)]
struct BusinessHours {
    sunday: Option<Vec<String>>,
    monday: Option<Vec<String>>,
    tuesday: Option<Vec<String>>,
    wednesday: Option<Vec<String>>,
    thursday: Option<Vec<String>>,
    friday: Option<Vec<String>>,
    saturday: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize)]
struct Link {
    url: String,
    text: String
}

impl Vendor {
    pub async fn find_by_id(coll: &Collection<Vendor>, user_id: ObjectId) -> Result<Vendor, AppError> {
        match coll.find_one(doc!{"_id": user_id}).await {
            Ok(Some(v)) => Ok(v),
            Ok(None) => Err(AppError::invalid_input("No Vendor with that ID")),
            Err(e) => Err(AppError::Database(e.into()))
        }
    }
}
