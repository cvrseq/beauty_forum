use actix_web::{web, App, HttpResponse, HttpServer, Responder}; 
use serde::{Deserialize, Serialize}; 
use std::sync::Mutex;
use rand::thread_rng; 
use rand::Rng;
//пересмотреть подход
#[derive(Deserialize)]
struct phoneNumberRequest { 
    phone: String,
}

#[derive(Deserialize)]
struct ConfirmCodeRequest {
    phone: String,
    code: String,
}

#[derive(Serialize)]
struct User {
    id: u32,
    phone: String,
}

struct AppState {
    users: Mutex<Vec<User>>,
    codes: Mutex<Vec<(String, String)>>, 
}

async fn send_code(data: web::Data<AppState>, req: web::Json<phoneNumberRequest>) -> impl Responder { 
    let code = thread_rng().gen_range(1000..=9999).to_string();
   
    {
        let mut codes = data.codes.lock().unwrap();
        codes.push((req.phone.clone(), code.clone()));
    }

    HttpResponse::Ok().body(format!("Code sent to {}", req.phone))
}


async fn confirm_code(data: web::Data<AppState>, req: web::Json<ConfirmCodeRequest>) -> impl Responder {
    let mut codes = data.codes.lock().unwrap();
    if let Some(pos) = codes.iter().position(|(phone, code)| phone == &req.phone && code == &req.code) {
        codes.remove(pos);
        let mut users = data.users.lock().unwrap();
        let new_user = User {
            id: (users.len() + 1) as u32,
            phone: req.phone.clone(),
        };
        users.push(new_user);
        return HttpResponse::Ok().json("Registration successful");
    }
    HttpResponse::BadRequest().body("Invalid code")
}

