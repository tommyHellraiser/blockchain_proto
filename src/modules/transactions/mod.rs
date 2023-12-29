use actix_web::web;

mod services;
mod data;

pub(in super::super) fn services(cfg: &mut web::ServiceConfig) {
    cfg.service(services::post_transaction);
}
