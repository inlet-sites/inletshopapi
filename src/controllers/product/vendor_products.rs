use actix_web::{HttpResponse, web, get};
use serde::Serialize;
use mongodb::{
    Database,
    bson::oid::ObjectId
};
use crate::{
    app_error::AppError,
    models::product::{Product, ShortProduct},
};

#[get("/vendor/{vendor_id}/product")]
pub async fn route(
    db: web::Data<Database>,
    path: web::Path<String>
) -> Result<HttpResponse, AppError> {
    let vendor_id = ObjectId::parse_str(path.into_inner())
        .map_err(|_| AppError::invalid_input("Invalid vendor ID"))?;
    let products = Product::find_by_vendor(&db, vendor_id).await?;
    let response_products: Vec<ResponseProduct> = products
        .into_iter()
        .map(|p| ResponseProduct::from_short_product(p)).collect();
    Ok(HttpResponse::Ok().json(response_products))
}

#[derive(Serialize)]
struct ResponseProduct {
    id: String,
    name: String,
    tags: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image: Option<String>,
    price: ResponsePrice
}

#[derive(Serialize)]
#[serde(untagged)]
enum ResponsePrice {
    Single(i32),
    Multi((i32, i32))
}

impl ResponseProduct {
    fn from_short_product(p: ShortProduct) -> ResponseProduct {
        let min_max = if p.prices.len() == 1 {
            ResponsePrice::Single(p.prices[0].price)
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
            ResponsePrice::Multi(sub_vec)
        };

        ResponseProduct {
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
