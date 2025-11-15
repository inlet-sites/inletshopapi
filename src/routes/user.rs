use actix_web::web;
use crate::controllers::user::vendors;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(vendors::get_many::route);
    cfg.service(vendors::get_one::route);
    cfg.service(vendors::products::get_many::route);
}
