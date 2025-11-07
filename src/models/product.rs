use serde::{Serialize, Deserialize};
use mongodb::bson::oid::ObjectId;

#[derive(Serialize, Deserialize)]
pub struct Product {
    pub _id: ObjectId,
    pub vendor: ObjectId,
    pub name: String,
    pub tags: Option<Vec<String>>,
    pub images: Option<Vec<String>>,
    pub active: bool,
    pub archived: bool,
    pub variations: Vec<Variation>
}

#[derive(Serialize, Deserialize)]
pub struct Variation {
    pub descriptor: String,
    pub price: i32,
    pub quantity: i32,
    pub shipping: i32,
    pub images: Option<Vec<String>>,
    pub price_id: Option<String>,
    pub purchase_option: String,
    pub archived: bool
}
