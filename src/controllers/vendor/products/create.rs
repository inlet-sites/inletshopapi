use actix_web::{HttpResponse, HttpRequest, web, post};
use serde::Deserialize;
use mongodb::{
    Database,
    bson::{DateTime, oid::ObjectId}
};
use crate::{
    app_error::AppError,
    auth::vendor_auth,
    models::product::{Product, Price, PurchaseOption},
    models::vendor::Vendor,
    dto::product::ProductVendorResponse
};

#[derive(Deserialize)]
struct Body {
    name: String,
    tags: Vec<String>,
    archived: bool,
    prices: Vec<BodyPrice>
}

#[derive(Deserialize)]
struct BodyPrice {
    descriptor: String,
    price: i32,
    quantity: i32,
    shipping: i32,
    purchase_option: PurchaseOption
}

#[post("/vendor/products")]
pub async fn route(
    db: web::Data<Database>,
    body: web::Json<Body>,
    req: HttpRequest
) -> Result<HttpResponse, AppError> {
    let vendor = vendor_auth(&db, &req).await?;
    let product = create_product(body.into_inner(), &vendor);
    product.insert(&db).await?;
    Ok(HttpResponse::Ok().json(ProductVendorResponse::from(product)))
}

fn create_product(body: Body, vendor: &Vendor) -> Product {
    let mut product = Product {
        _id: ObjectId::new(),
        vendor: vendor._id,
        name: body.name,
        tags: body.tags,
        images: Vec::new(),
        active: true,
        archived: body.archived,
        created_at: DateTime::now(),
        prices: Vec::new()
    };

    for price in body.prices {
        product.prices.push(Price {
            _id: ObjectId::new(),
            descriptor: price.descriptor,
            price: price.price,
            quantity: price.quantity,
            shipping: price.shipping,
            images: Vec::new(),
            purchase_option: if let Some(_) = vendor.stripe {
                price.purchase_option
            } else {
                PurchaseOption::List
            },
            archived: false
        });
    }

    product
}
