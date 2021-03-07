use crate::user::handlers;
use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    // cfg.service(handlers::show_users);
    cfg.service(handlers::show_user);
    cfg.service(handlers::add_user);
    // cfg.service(handlers::update);
    // cfg.service(handlers::delete);
}
