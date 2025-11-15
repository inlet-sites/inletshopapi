use serde::Serialize;
use crate::models::product::ShortProduct;

#[derive(Serialize)]
pub struct ShortProductResponse {
    id: String,
    name: String,
    tags: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image: Option<String>,
    price: ShortPriceResponse
}

#[derive(Serialize)]
#[serde(untagged)]
enum ShortPriceResponse {
    Single(i32),
    Multi((i32, i32))
}

impl ShortProductResponse {
    pub fn from_short_product(p: ShortProduct) -> ShortProductResponse {
        let min_max = if p.prices.len() == 1 {
            ShortPriceResponse::Single(p.prices[0].price)
        } else {
            let mut sub_vec = (p.prices[0].price, p.prices[0].price);
            for p_obj in p.prices {
                if p_obj.price < sub_vec.0 {
                    sub_vec.0 = p_obj.price;
                }
                if p_obj.price > sub_vec.1 {
                    sub_vec.1 = p_obj.price;
                }
            }
            ShortPriceResponse::Multi(sub_vec)
        };

        ShortProductResponse {
            id: p._id.to_string(),
            name: p.name,
            tags: p.tags,
            image: if p.images.len() == 0 {
                None
            } else {
                Some(p.images[0].clone())
            },
            price: min_max
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::product::ShortPrice;
    use mongodb::bson::oid::ObjectId;

    #[test]
    fn creates_single_price_response() {
        let short_product = ShortProduct {
            _id: ObjectId::new(),
            name: String::from("Item"),
            tags: vec![String::from("one"), String::from("two")],
            images: vec![String::from("linky")],
            prices: vec![ShortPrice {
                price: 1299
            }]
        };

        let result = ShortProductResponse::from_short_product(short_product);
        assert_eq!(result.name, "Item");
        assert_eq!(result.tags.len(), 2);
        assert!(matches!(result.price, ShortPriceResponse::Single(_)));
    }

    #[test]
    fn creates_multi_price_response() {
        let short_product = ShortProduct {
            _id: ObjectId::new(),
            name: String::from("Item"),
            tags: vec![String::from("three")],
            images: Vec::new(),
            prices: vec![
                ShortPrice {
                    price: 1299
                },
                ShortPrice {
                    price: 1587
                },
                ShortPrice {
                    price: 123
                },
                ShortPrice {
                    price: 777
                }
            ]
        };

        let result = ShortProductResponse::from_short_product(short_product);
        assert_eq!(result.name, "Item");
        assert_eq!(result.tags.len(), 1);
        assert!(matches!(result.price, ShortPriceResponse::Multi((123, 1587))));
    }
}
