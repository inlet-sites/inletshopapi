use actix_web::web;
use crate::controllers::vendor::{
    create_password,
    change_password,
    login,
    logout,
    me,
    update,
    update_thumbnail,
    password_email,
    reset_password,
    create_connect,
    create_session,

    products
};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(create_password::route);
    cfg.service(change_password::route);
    cfg.service(login::route);
    cfg.service(logout::route);
    cfg.service(me::route);
    cfg.service(update::route);
    cfg.service(update_thumbnail::route);
    cfg.service(password_email::route);
    cfg.service(reset_password::route);
    cfg.service(create_connect::route);
    cfg.service(create_session::route);

    cfg.service(products::create::route);
    cfg.service(products::delete::route);
    cfg.service(products::get_many::route);
}
