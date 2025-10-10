use actix_web::web;
use crate::controllers::vendor::{
    create_password,
    change_password,
    login,
    get_vendor_self,
    get_vendor
};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(create_password::route);
    cfg.service(change_password::route);
    cfg.service(login::route);
    cfg.service(get_vendor_self::route);
    cfg.service(get_vendor::route);
}
