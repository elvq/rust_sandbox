use actix_web::web;
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FullNumber {

    pub first_number : i32,
    pub second_number  : i32,
}


impl From<web::Json<FullNumber>> for FullNumber {
    fn from(fullnumber : web::Json<FullNumber>) -> Self {

        FullNumber {

           first_number : fullnumber.first_number,
           second_number  : fullnumber.second_number,
        }


    }

}




