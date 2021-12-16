use actix_web::web;
use super::handlers::*;




pub fn id_route(cfg : &mut web::ServiceConfig) {

cfg
.route("/who", web::post().to(id_handler));

}

