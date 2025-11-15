use actix_web::web;
use crate::controllers::product::{
    delete,
    vendor_products
};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(delete::route);
    cfg.service(vendor_products::route);
}
