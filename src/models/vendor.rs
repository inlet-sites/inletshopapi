use serde::{Serialize, Deserialize};
use mongodb::{
    bson::{oid::ObjectId, DateTime, Document, doc},
    Collection
};

use crate::app_error::AppError;

#[derive(Serialize, Deserialize)]
pub struct Vendor {
    pub _id: ObjectId,
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
pub struct StripeData {
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
    pub async fn find_by_id(coll: &Collection<Vendor>, vendor_id: ObjectId) -> Result<Vendor, AppError> {
        match coll.find_one(doc!{"_id": vendor_id}).await {
            Ok(Some(v)) => Ok(v),
            Ok(None) => Err(AppError::not_found("Vendor with this ID not found")),
            Err(e) => Err(AppError::Database(e.into()))
        }
    }

    pub async fn update(self, coll: &Collection<Vendor>, data: Document) -> Result<Vendor, AppError> {
        match coll.find_one_and_update(doc!{"_id": self._id}, doc!{"$set": data}).await? {
            Some(v) => Ok(v),
            None => Err(AppError::not_found("User with this ID does not exist"))
        }
    }
}
