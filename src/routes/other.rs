use actix_web::web;
use crate::controllers::other::{
    documentation_route,
    documents_route
};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(documentation_route);
    cfg.service(documents_route);
}
