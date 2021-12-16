use actix_web:: {web, HttpResponse};
use super::state::AppState;



pub async fn handler_count (app_state: web::Data<AppState>) -> HttpResponse {


let message = &app_state.message;

let mut count =app_state
.count
.lock()
.unwrap();

let response = format!("{} {} fois", message, count);

*count +=1;

        HttpResponse::Ok().json(&response)


}