use actix_web::{HttpResponse, web};
use actix_web::http::StatusCode;
use super::models::FullNumber;
use std::io;

//use models::FullNumber;


pub async fn handler_firstname  (
     param : web::Path<String>)           -> HttpResponse {

        let firstname= param.0;               
        let message=format!("your firstname is : {}", firstname);
HttpResponse::Ok().json(message)


} 


pub async fn handler_fullnumber  (
    fullnumber : web::Json<FullNumber>) -> HttpResponse {
       println!("Bien reçu!");
       //let firstname= &fullname.first_name;
       //let lastname= &fullname.last_name;                      
       //let message=format!("Your full name is : {} {} !!!", firstname, lastname);
       //HttpResponse::Ok().json(message)
       HttpResponse::Ok().json("Message reçu!!")


} 




#[actix_rt::test]
async fn firstname_test () {

        let firstname : web::Path <String> = web::Path::from("eric".to_string());

        let resp = handler_firstname(firstname).await;

        assert_eq!(resp.status(), StatusCode::OK);

}



#[actix_rt::test]
async fn fullnumber_test () {

        let fullnumber  = web::Json(FullNumber {

                first_number : 13,
                second_number  : 28,

        });
             

        let resp = handler_fullnumber(fullnumber).await;

        assert_eq!(resp.status(), StatusCode::OK);

}