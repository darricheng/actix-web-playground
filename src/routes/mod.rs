use super::controllers;
use actix_web::web;

mod comedian_routes;

pub fn config_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(comedian_routes::get_data);
}
