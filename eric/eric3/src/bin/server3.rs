use actix_web::{web, App, HttpServer};
use std::io;
use std::sync::Mutex;



#[path="../handlers.rs"]
mod handlers;

#[path="../routes.rs"]
mod routes;


#[path ="../state.rs"]
mod state;


use routes::*;
use state::AppState;

#[actix_rt::main]
async fn main() -> io::Result<()> {

//Web::Data <> object instantiation

let shared_data= web::Data::new(AppState

{
    message : "Tu viens ici depuis ".to_string(),
    count   : Mutex::new(0),
});


//Application instatiation withj shared data

let app= move || App::new().app_data(shared_data.clone()).configure(route_count);

// HTTP server Start
HttpServer::new(app).bind("127.0.0.1:3300")?.run().await

}
