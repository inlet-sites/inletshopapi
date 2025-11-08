use actix_web::web;
use crate::controllers::product::{
    create
};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(create::route);
}
