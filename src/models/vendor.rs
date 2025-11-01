use serde::{Serialize, Deserialize};
use mongodb::{
    bson::{oid::ObjectId, DateTime, Document, doc},
    Database
};
use futures::stream::TryStreamExt;
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
    pub phone: Option<String>,
    pub email: Option<String>,
    pub address: Option<Address>,
    pub slogan: Option<String>,
    pub description: Option<String>,
    pub image: Option<String>,
    pub hours: Option<BusinessHours>,
    pub links: Option<Vec<Link>>,
    pub website: Option<String>
}

#[derive(Serialize, Deserialize)]
pub struct StripeData {
    pub account_id: String,
    pub activated: bool
}

#[derive(Serialize, Deserialize)]
pub struct Address {
    pub text: Option<String>,
    pub link: Option<String>
}

#[derive(Serialize, Deserialize)]
pub struct BusinessHours {
    sunday: Option<Vec<String>>,
    monday: Option<Vec<String>>,
    tuesday: Option<Vec<String>>,
    wednesday: Option<Vec<String>>,
    thursday: Option<Vec<String>>,
    friday: Option<Vec<String>>,
    saturday: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize)]
pub struct Link {
    url: String,
    text: String
}

#[derive(Serialize)]
pub struct ResponseVendor {
    id: String,
    email: String,
    owner: String,
    store: String,
    url: String,
    public_data: PublicData,
    html: Option<String>,
    new_order_send_email: bool
}

impl Vendor {
    pub async fn get_all(db: &Database, projection_doc: Document) -> Result<Vec<Document>, AppError> {
        let cursor = db.collection::<Document>("vendors")
            .find(doc!{})
            .projection(projection_doc)
            .await?;

        let vendors: Vec<Document> = cursor.try_collect().await?;
        Ok(vendors)
    }

    pub async fn find_by_id(db: &Database, vendor_id: ObjectId) -> Result<Vendor, AppError> {
        match db.collection::<Vendor>("vendors").find_one(doc!{"_id": vendor_id}).await {
            Ok(Some(v)) => Ok(v),
            Ok(None) => Err(AppError::not_found("Vendor with this ID not found")),
            Err(e) => Err(AppError::Database(e.into()))
        }
    }

    pub async fn find_by_email(db: &Database, email: &String) -> Result<Vendor, AppError> {
        match db.collection::<Vendor>("vendors").find_one(doc!{"email": email}).await {
            Ok(Some(v)) => Ok(v),
            Ok(None) => Err(AppError::not_found("Vendor with this email does not exist")),
            Err(e) => Err(AppError::Database(e.into()))
        }
    }

    pub async fn find_by_url(db: &Database, url: &String) -> Result<Vendor, AppError> {
        match db.collection::<Vendor>("vendors").find_one(doc!{"url": url}).await {
            Ok(Some(v)) => Ok(v),
            Ok(None) => Err(AppError::not_found("The page you are looking for doesn't exist")),
            Err(e) => Err(AppError::Database(e.into()))
        }
    }

    pub async fn update(&self, db: &Database, data: Document) -> Result<Vendor, AppError> {
        match db.collection::<Vendor>("vendors").find_one_and_update(doc!{"_id": self._id}, doc!{"$set": data}).await? {
            Some(v) => Ok(v),
            None => Err(AppError::not_found("User with this ID does not exist"))
        }
    }

    pub fn response(self) -> ResponseVendor {
        ResponseVendor {
            id: self._id.to_string(),
            email: self.email,
            owner: self.owner,
            store: self.store,
            url: self.url,
            public_data: self.public_data,
            html: self.html,
            new_order_send_email: self.new_order_send_email
        }
    }
}
