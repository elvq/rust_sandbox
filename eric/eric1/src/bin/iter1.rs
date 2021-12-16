use actix_web::{HttpServer, App};
use std::io;


#[path="../routes.rs"]
mod routes;

#[path="../handlers.rs"]
mod handlers;

#[path="../models.rs"]
mod models;

use routes::*;

#[actix_rt::main]
async fn main() -> io::Result<()> {

let app = move || App::new().configure(route_firstname);

HttpServer::new(app).bind("127.0.0.1:9000")?.run().await


}