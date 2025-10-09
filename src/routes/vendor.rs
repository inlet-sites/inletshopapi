use actix_web::web;
use crate::controllers::vendor::{
    create_password
};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(create_password::route);
}
