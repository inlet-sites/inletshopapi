use serde::{Serialize, Deserialize};
use mongodb::bson::{Document, doc};
use crate::models::product::Price;

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

impl VendorResponse {
    pub fn projection() -> Document {
        doc!{
            "$project": {
                "_id": "$prices._id",
                "descriptor": "$prices.descriptor",
                "price": "$prices.price",
                "quantity": "$prices.quantity",
                "shipping": "$prices.shipping",
                "images": "$prices.images",
                "purchase_option": "$prices.purchase_option"
            }
        }
    }
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

impl From<Document> for VendorResponse {
    fn from(d: Document) -> Self {
        Self {
            id: d.get_object_id("_id").unwrap().to_string(),
            descriptor: d.get_str("descriptor").unwrap().to_string(),
            price: d.get_i32("price").unwrap(),
            quantity: d.get_i32("quantity").unwrap(),
            shipping: d.get_i32("shipping").unwrap(),
            images: d.get_array("images").unwrap()
                .iter()
                .map(|u| u.as_str().unwrap().to_string())
                .collect(),
            purchase_option: d.get_str("purchase_option").unwrap().to_string()
        }
    }
}
