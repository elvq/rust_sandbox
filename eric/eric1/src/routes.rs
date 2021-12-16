use actix_web::{web};
use super::handlers::*;



pub fn route_firstname (cfg: &mut web::ServiceConfig) {

cfg.route("{name}", web::get().to(handler_firstname));

}


pub fn route_fullnumber (cfg: &mut web::ServiceConfig) {

    cfg
    .service(web::scope("/numbers"))
    .route("/", web::post().to(handler_fullnumber));
    
    }