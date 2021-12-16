//Module Imports

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::io;


//Configure routes

pub fn unique_route(cfg: &mut web::ServiceConfig) {

    cfg.route("/", web::get().to(unique_handler));
}


//Configure handler

pub async fn unique_handler () -> impl Responder {

HttpResponse::Ok().json(" Eric tu es arrivé sur l'index !!")

}


//Instantiate and run the Http Server


#[actix_rt::main]
async fn main() -> io::Result<()> {

//Construct App and configure routes

let app = move || App::new().configure(unique_route);

//Start HTTP Server

HttpServer::new(app).bind("127.0.0.1:5000")?.run().await


}



