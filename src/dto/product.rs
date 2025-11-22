use serde::{Serialize, Deserialize};
use mongodb::bson::{Document, DateTime, doc, oid::ObjectId};
use crate::models::product::{Product, PurchaseOption};

#[derive(Serialize, Deserialize)]
pub struct ProductVendorDb {
    _id: ObjectId,
    name: String,
    tags: Vec<String>,
    images: Vec<String>,
    thumbnail: Option<String>,
    archived: bool,
    created_at: DateTime,
    prices: Vec<PriceVendorDb>
}

#[derive(Serialize, Deserialize)]
struct PriceVendorDb {
    _id: ObjectId,
    descriptor: String,
    price: i32,
    quantity: i32,
    shipping: i32,
    images: Vec<String>,
    purchase_option: PurchaseOption,
    archived: bool
}

impl ProductVendorDb {
    pub fn projection() -> Document {
        doc!{
            "_id": 1,
            "name": 1,
            "tags": 1,
            "images": 1,
            "thumbnail": 1,
            "archived": 1,
            "created_at": 1,
            "prices._id": 1,
            "prices.descriptor": 1,
            "prices.price": 1,
            "prices.quantity": 1,
            "prices.shipping": 1,
            "prices.images": 1,
            "prices.purchase_option": 1,
            "prices.archived": 1
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct ProductVendorResponse {
    id: String,
    name: String,
    tags: Vec<String>,
    images: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    thumbnail: Option<String>,
    archived: bool,
    created_at: String,
    prices: Vec<PriceVendorResponse>
}

#[derive(Serialize, Deserialize)]
struct PriceVendorResponse {
    id: String,
    descriptor: String,
    price: i32,
    quantity: i32,
    shipping: i32,
    images: Vec<String>,
    purchase_option: String,
    archived: bool
}

impl From<ProductVendorDb> for ProductVendorResponse {
    fn from(p: ProductVendorDb) -> ProductVendorResponse {
        ProductVendorResponse {
            id: p._id.to_string(),
            name: p.name,
            tags: p.tags,
            images: p.images,
            thumbnail: p.thumbnail,
            archived: p.archived,
            created_at: p.created_at.to_string(),
            prices: p.prices.into_iter()
                .map(|price| PriceVendorResponse {
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

impl From<Product> for ProductVendorResponse {
    fn from(p: Product) -> ProductVendorResponse {
        ProductVendorResponse {
            id: p._id.to_string(),
            name: p.name,
            tags: p.tags,
            images: p.images,
            thumbnail: p.thumbnail,
            archived: p.archived,
            created_at: p.created_at.to_string(),
            prices: p.prices.into_iter()
                .map(|price| PriceVendorResponse {
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
    thumbnail: Option<String>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    thumbnail: Option<String>,
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
            thumbnail: p.thumbnail,
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
            "thumbnail": 1,
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

#[derive(Deserialize)]
pub struct ProductShortDb {
    _id: ObjectId,
    name: String,
    tags: Vec<String>,
    thumbnail: Option<String>,
    prices: Vec<PriceShortDb>
}

#[derive(Deserialize)]
pub struct PriceShortDb {
    price: i32
}

impl ProductShortDb {
    pub fn projection() -> Document {
        doc!{
            "_id": 1,
            "name": 1,
            "tags": 1,
            "thumbnail": 1,
            "prices.price": 1
        }
    }
}

#[derive(Serialize)]
pub struct ProductShortResponse {
    id: String,
    name: String,
    tags: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    thumbnail: Option<String>,
    price: PriceShortResponse
}

#[derive(Serialize)]
#[serde(untagged)]
enum PriceShortResponse {
    Single(i32),
    Multi((i32, i32))
}

impl From<ProductShortDb> for ProductShortResponse {
    fn from(p: ProductShortDb) -> Self {
        let get_min_max_price = |p: Vec<PriceShortDb>| -> PriceShortResponse {
            if p.len() == 1 {
                PriceShortResponse::Single(p[0].price)
            } else {
                let mut results = (p[0].price, p[0].price);
                for v in p {
                    if v.price < results.0 {
                        results.0 = v.price
                    }
                    if v.price > results.1 {
                        results.1 = v.price
                    }
                }
                PriceShortResponse::Multi(results)
            }
        };

        ProductShortResponse {
            id: p._id.to_string(),
            name: p.name,
            tags: p.tags,
            thumbnail: p.thumbnail,
            price: get_min_max_price(p.prices)
        }
    }
}
