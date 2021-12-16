use super::handlers::*;
use actix_web::web;


//Config of post route

pub fn course_routes (cfg: &mut web::ServiceConfig) {

    cfg
    .service(web::scope("/courses")
    .route("/", web::post().to(new_course)));
}

*/


//Config of count route

pub fn route_count (cfg: &mut web::ServiceConfig) {

    cfg.route("/count", web::get().to(handler_count));
}

