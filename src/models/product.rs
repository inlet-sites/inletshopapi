use serde::{Serialize, Deserialize};
use mongodb::{
    Database,
    bson::{DateTime, doc, oid::ObjectId}
};
use crate::app_error::AppError;

#[derive(Serialize, Deserialize)]
pub struct Product {
    pub _id: ObjectId,
    pub vendor: ObjectId,
    pub name: String,
    pub tags: Vec<String>,
    pub images: Vec<String>,
    pub active: bool,
    pub archived: bool,
    pub created_at: DateTime,
    pub prices: Vec<Price>,
}

#[derive(Serialize, Deserialize)]
pub struct Price {
    pub _id: ObjectId,
    pub descriptor: String,
    pub price: i32,
    pub quantity: i32,
    pub shipping: i32,
    pub images: Vec<String>,
    pub purchase_option: PurchaseOption,
    pub archived: bool
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PurchaseOption {
    Ship,
    Buy,
    List
}

impl Product {
    pub async fn insert(&self, db: &Database) -> Result<(), AppError> {
        match db.collection::<Product>("products").insert_one(self).await {
            Ok(_) => Ok(()),
            Err(e) => Err(AppError::Database(e.into()))
        }
    }
}
