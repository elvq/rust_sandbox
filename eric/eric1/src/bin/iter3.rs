use actix_web::{web, App, HttpServer};
use serde::{Serialize, Deserialize};
use std::io;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Info {
    username: i32,
}

/// deserialize `Info` from request's body
async fn index(info: web::Json<Info>) -> String {
    format!("Welcome {}!", info.username)
}

//New method used

#[actix_rt::main]
async fn main() -> io::Result<()>  {
    let app = move || App::new()
    .service(web::resource("/index.html")
    .route(web::post().to(index))
    );


    HttpServer::new(app).bind("127.0.0.1:9000")?.run().await
}



