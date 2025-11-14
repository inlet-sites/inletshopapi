use actix_web::web;
use crate::controllers::user::{
    vendors::{
        get_many,
        get_one
    }
};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_many::route);
    cfg.service(get_one::route);
}
