//Import Modules

use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use std::io;


//Configure route

pub fn error_route (cfg: &mut web::ServiceConfig) {

    cfg.route("/error", web::get().to(error_handler));

}



//Configure Handler

pub async fn error_handler() -> impl Responder {

    HttpResponse::Ok().json("This is my ERROR page, damn right Eric.")

}


//Instantiate HTTP server and start it

#[actix_rt::main]
async fn main() -> io::Result<()> {

    //Instantiate Web Application 
    let app= move || App::new().configure(error_route);

    //Start HTTP server on chosen IP:Port

    HttpServer::new(app).bind("127.0.0.1:6152")?.run().await


}