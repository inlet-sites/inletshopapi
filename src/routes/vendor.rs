use actix_web::web;
use crate::controllers::vendor::{
    login_route
};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(login_route);
}
