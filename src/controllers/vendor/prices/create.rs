use actix_web::{HttpResponse, HttpRequest, web, post};
use mongodb::{Database, bson::{Document, doc, oid::ObjectId}};
use serde::Deserialize;
use crate::{
    app_error::AppError,
    auth::vendor_auth,
    models::product::{Product, Price, PurchaseOption},
    dto::price::VendorResponse
};

#[derive(Deserialize)]
struct Body {
    descriptor: String,
    price: i32,
    quantity: i32,
    shipping: i32,
    purchase_option: PurchaseOption
}

#[post("/vendor/products/{product_id}/prices")]
pub async fn route(
    db: web::Data<Database>,
    path: web::Path<String>,
    body: web::Json<Body>,
    req: HttpRequest
) -> Result<HttpResponse, AppError> {
    let vendor = vendor_auth(&db, &req).await?;

    let product_id = ObjectId::parse_str(path.into_inner())
        .map_err(|_| AppError::invalid_input("Invalid product ID"))?;

    let Body {descriptor, price, quantity, shipping, purchase_option} = body.into_inner();
    let price = Price::new(descriptor, price, quantity, shipping, purchase_option);
    let update_doc = create_update_doc(&price);

    Product::update(&db, product_id, Some(vendor._id), update_doc).await?;

    let response_price: VendorResponse = price.into();
    Ok(HttpResponse::Ok().json(response_price))
}

fn create_update_doc(p: &Price) -> Document {
    doc!{"$push": {
        "prices": {
            "_id": p._id.to_string(),
            "descriptor": &p.descriptor,
            "price": p.price,
            "quantity": p.quantity,
            "shipping": p.shipping,
            "images": &p.images,
            "purchase_option": p.purchase_option.to_string(),
            "archived": p.archived
        }
    }}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_update_doc() {
        let price = Price::new("test".to_string(), 1299, 12, 499, PurchaseOption::Ship);

        let result = create_update_doc(&price);
        let push = result.get_document("$push").unwrap();
        let prices = push.get_document("prices").unwrap();

        assert_eq!(prices.get_str("descriptor").unwrap(), "test".to_string());
        assert_eq!(prices.get_i32("price").unwrap(), 1299);
        assert_eq!(prices.get_i32("quantity").unwrap(), 12);
        assert_eq!(prices.get_i32("shipping").unwrap(), 499);
    }
}
