use super::handlers::*;
use actix_web::web;


//Config of index route
/*
pub fn route_index (cfg: &mut web::ServiceConfig) {

    cfg.route("/", web::get().to(handler_index));
}

*/


//Config of count route

pub fn route_count (cfg: &mut web::ServiceConfig) {

    cfg.route("/count", web::get().to(handler_count));
}

