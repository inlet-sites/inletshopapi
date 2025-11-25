use serde::{Serialize, Deserialize};
use mongodb::bson::{Document, doc};
use crate::models::product::{Price, PurchaseOption};

#[derive(Deserialize)]
pub struct VendorDb {
    pub _id: String,
    pub descriptor: String,
    pub price: i32,
    pub quantity: i32,
    pub shipping: i32,
    pub images: Vec<String>,
    pub purchase_option: PurchaseOption
}

impl VendorDb {
    pub fn projection() -> Document {
        doc!{
            "_id": 1,
            "descriptor": 1,
            "price": 1,
            "quantity": 1,
            "shipping": 1,
            "images": 1,
            "purchase_option": 1
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct VendorResponse {
   pub id: String,
   pub descriptor: String,
   pub price: i32,
   pub quantity: i32,
   pub shipping: i32,
   pub images: Vec<String>,
   pub purchase_option: String
}

impl From<Price> for VendorResponse {
    fn from(p: Price) -> Self {
        Self {
            id: p._id.to_string(),
            descriptor: p.descriptor,
            price: p.price,
            quantity: p.quantity,
            shipping: p.shipping,
            images: p.images,
            purchase_option: p.purchase_option.into()
        }
    }
}

impl From<VendorDb> for VendorResponse {
    fn from(p: VendorDb) -> Self {
        Self {
            id: p._id.to_string(),
            descriptor: p.descriptor,
            price: p.price,
            quantity: p.quantity,
            shipping: p.shipping,
            images: p.images,
            purchase_option: p.purchase_option.into()
        }
    }
}
