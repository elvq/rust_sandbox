use actix_web::{web, App, HttpServer};
use serde::{Serialize, Deserialize};
use std::io;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Info {
    username: String,
    job     : String,
}

/// deserialize `Info` from request's body
async fn index(info: web::Json<Info>) -> String {
    println!("Bien reçu!!");
    format!("Welcome {}!. You are a {}.", info.username, info.job)
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



