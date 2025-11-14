use actix_web::web;
use crate::controllers::vendor::{
    create_password,
    change_password,
    login,
    get_vendor_self,
    get_vendor,
    update,
    update_image,
    password_email,
    reset_password,
    create_connect,
    create_session
};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(create_password::route);
    cfg.service(change_password::route);
    cfg.service(login::route);
    cfg.service(get_vendor_self::route);
    cfg.service(get_vendor::route);
    cfg.service(update::route);
    cfg.service(update_image::route);
    cfg.service(password_email::route);
    cfg.service(reset_password::route);
    cfg.service(create_connect::route);
    cfg.service(create_session::route);
}
