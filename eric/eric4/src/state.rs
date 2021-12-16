use  std::sync::Mutex;


//Definition of custom Application State Struct


pub struct AppState {

    pub message : String,
    pub count   : Mutex<u32>,

}