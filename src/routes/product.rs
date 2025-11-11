use actix_web::web;
use crate::controllers::product::{
    create,
    delete
};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(create::route);
    cfg.service(delete::route);
}
