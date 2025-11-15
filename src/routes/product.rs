use actix_web::web;
use crate::controllers::product::{
    vendor_products
};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(vendor_products::route);
}
