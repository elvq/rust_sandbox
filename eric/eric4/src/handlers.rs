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



pub async fn new_course(
        new_course: web::Json<Course>,
        app_state: web::Data<AppState>,
        ) -> HttpResponse {
        println!("Received new course");
        let course_count_for_user = app_state
         .courses
         .lock()
        .unwrap()
        .clone()
        .into_iter()
        .filter(|course| course.tutor_id == new_course.tutor_id)
        .collect::<Vec<Course>>()
        .len();
        let new_course = Course {
        tutor_id: new_course.tutor_id,
        course_id: Some(course_count_for_user + 1),
        course_name: new_course.course_name.clone(),
        posted_time: Some(Utc::now().naive_utc()),
        };
        app_state.courses.lock().unwrap().push(new_course);
        HttpResponse::Ok().json("Added course")
        }