use serde::Serialize;
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
