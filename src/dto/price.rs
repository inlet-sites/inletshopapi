use serde::{Serialize, Deserialize};
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
