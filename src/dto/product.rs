use serde::{Serialize, Deserialize};
use mongodb::bson::{Document, doc, oid::ObjectId};
use crate::models::product::{Product, PurchaseOption};

#[derive(Serialize)]
pub struct VendorResponse {
    pub id: String,
    pub name: String,
    pub tags: Vec<String>,
    pub images: Vec<String>,
    pub archived: bool,
    pub created_at: String,
    pub prices: Vec<VendorResponsePrice>
}

#[derive(Serialize)]
pub struct VendorResponsePrice {
    pub id: String,
    pub descriptor: String,
    pub price: i32,
    pub quantity: i32,
    pub shipping: i32,
    pub images: Vec<String>,
    pub purchase_option: String,
    pub archived: bool
}

impl VendorResponse {
    pub fn from_vendor(p: Product) -> VendorResponse {
        VendorResponse {
            id: p._id.to_string(),
            name: p.name,
            tags: p.tags,
            images: p.images,
            archived: p.archived,
            created_at: p.created_at.to_string(),
            prices: p.prices.into_iter()
                .map(|price| VendorResponsePrice {
                    id: price._id.to_string(),
                    descriptor: price.descriptor,
                    price: price.price,
                    quantity: price.quantity,
                    shipping: price.shipping,
                    images: price.images,
                    purchase_option: match price.purchase_option {
                        PurchaseOption::Ship => String::from("ship"),
                        PurchaseOption::Buy => String::from("buy"),
                        PurchaseOption::List => String::from("list")
                    },
                    archived: price.archived
                })
                .collect()
        }
    }
}

#[derive(Deserialize)]
pub struct ProductDb {
    _id: ObjectId,
    vendor: ObjectId,
    name: String,
    tags: Vec<String>,
    images: Vec<String>,
    prices: Vec<PriceDb>
}

#[derive(Deserialize)]
pub struct PriceDb {
    _id: ObjectId,
    descriptor: String,
    price: i32,
    quantity: i32,
    shipping: i32,
    images: Vec<String>,
    purchase_option: PurOptDb
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PurOptDb {
    Ship,
    Buy,
    List
}

#[derive(Serialize)]
pub struct ProductResponse {
    id: String,
    vendor: String,
    name: String,
    tags: Vec<String>,
    images: Vec<String>,
    prices: Vec<PriceResponse>
}

#[derive(Serialize)]
pub struct PriceResponse {
    id: String,
    descriptor: String,
    price: i32,
    quantity: i32,
    shipping: i32,
    images: Vec<String>,
    purchase_option: PurOptDb
}

impl From<ProductDb> for ProductResponse {
    fn from(p: ProductDb) -> Self {
        ProductResponse {
            id: p._id.to_string(),
            vendor: p.vendor.to_string(),
            name: p.name,
            tags: p.tags,
            images: p.images,
            prices: p.prices.into_iter().map(|pr| {
                PriceResponse {
                    id: pr._id.to_string(),
                    descriptor: pr.descriptor,
                    price: pr.price,
                    quantity: pr.quantity,
                    shipping: pr.shipping,
                    images: pr.images,
                    purchase_option: pr.purchase_option
                }
            }).collect()
        }
    }
}

impl ProductDb {
    pub fn projection() -> Document {
        doc!{
            "_id": 1,
            "vendor": 1,
            "name": 1,
            "tags": 1,
            "images": 1,
            "prices._id": 1,
            "prices.descriptor": 1,
            "prices.price": 1,
            "prices.quantity": 1,
            "prices.shipping": 1,
            "prices.images": 1,
            "prices.purchase_option": 1
        }
    }
}
