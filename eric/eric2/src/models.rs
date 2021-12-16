use actix_web::web;
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Id {


   pub  name: String,
   pub  age : i32,
}



impl From<web::Json<Id>> for Id {
    fn from(id : web::Json<Id>) -> Self {
        Id {
           name : id.name.clone(),
           age  : id.age,
        }
    }
}





