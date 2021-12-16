use actix_web::{web, HttpResponse};
use actix_web::http::StatusCode;
use super::models::*;
//use std::io;



pub  async  fn id_handler (id : web::Json<Id>) -> HttpResponse  {

    println!("Received request !!");

    let message=format!("Your name is {}. And you are {} years old.", id.name, id.age);

    HttpResponse::Ok().json(message)

}




#[actix_rt::test]
async fn id_test() {


    let id = web::Json(Id {

        name : "eric".to_string(),
        age  :50,
    });

    let resp = id_handler (id).await;

    assert_eq!(resp.status(), StatusCode::OK);


}