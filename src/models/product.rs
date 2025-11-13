use serde::{Serialize, Deserialize};
use mongodb::{
    Database,
    bson::{DateTime, Document, doc, oid::ObjectId}
};
use futures::stream::TryStreamExt;
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

#[derive(Deserialize)]
pub struct ShortProduct {
    pub _id: ObjectId,
    pub name: String,
    pub tags: Vec<String>,
    pub images: Vec<String>,
    pub prices: Vec<ShortPrice>
}

#[derive(Deserialize)]
pub struct ShortPrice {
    pub price: i32
}

impl Product {
    pub async fn insert(&self, db: &Database) -> Result<(), AppError> {
        match db.collection::<Product>("products").insert_one(self).await {
            Ok(_) => Ok(()),
            Err(e) => Err(AppError::Database(e.into()))
        }
    }

    pub async fn find_by_id(db: &Database, id: ObjectId) -> Result<Product, AppError> {
        match db.collection::<Product>("products").find_one(doc!{"_id": id}).await {
            Ok(Some(p)) => Ok(p),
            Ok(None) => Err(AppError::not_found("Product with this ID does not exist")),
            Err(e) => Err(AppError::Database(e.into()))
        }
    }

    pub async fn find_by_vendor(
        db: &Database,
        vendor_id: ObjectId,
        page: u64,
        results: i64
    ) -> Result<Vec<ShortProduct>, AppError> {
        let projection = doc!{
            "_id": 1,
            "name": 1,
            "tags": 1,
            "images": 1,
            "prices.price": 1
        };

        let cursor = db.collection::<ShortProduct>("products")
            .find(doc!{"vendor": vendor_id})
            .skip(page)
            .limit(results)
            .projection(projection)
            .await?;

        let products: Vec<ShortProduct> = cursor.try_collect().await?;
        Ok(products)
    }

    pub async fn update(&self, db: &Database, update_doc: Document) -> Result<Product, AppError> {
        match db.collection::<Product>("products").find_one_and_update(doc!{"_id": self._id}, doc!{"$set": update_doc}).await? {
            Some(p) => Ok(p),
            None => Err(AppError::not_found("Product with this ID does not exist"))
        }
    }

    pub fn is_owned(&self, vendor_id: &ObjectId) -> Result<(), AppError> {
        if self.vendor == *vendor_id {
            Ok(())
        } else {
            Err(AppError::forbidden("Unauthorized to edit this product"))
        }
    }
}
