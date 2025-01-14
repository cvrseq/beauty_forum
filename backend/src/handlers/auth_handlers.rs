use actix_web::{web, HttpResponse, Responder}; 
use serde::{Deserialize, Serialize}; 
use serde_json;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};

//пересмотреть подход
#[derive(Deserialize, Serialize)]
pub struct RegisterRequest { 
    pub phone: String,
}

#[derive(Deserialize, Serialize)]
pub struct RegisterResponse {
    pub message: String,
}

pub async fn register(data: web::Json<RegisterRequest>) -> impl Responder { 
    let path = "users.json"; 
    let req = data.into_inner();

    let mut users: Vec<RegisterRequest> = Vec::new(); 
    if let Ok(mut file) = File::open(path) { 
        let mut content = String::new();
        if file.read_to_string(&mut content).is_ok() && !content.trim().is_empty() { 
            if let Ok(parsed) = serde_json::from_str::<Vec<RegisterRequest>>(&content) { 
                users = parsed;
            }
        }
    }
    users.push(req);

    if let Ok(mut file) = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)
        {
            let new_content = serde_json::to_string_pretty(&users).unwrap_or_else(|_| "[]".to_string());
            if let Err(e) = file.write_all(new_content.as_bytes()) {
                return HttpResponse::InternalServerError().body(format!("Ошибка записи файла: {}", e));
            }
        } else {
            return HttpResponse::InternalServerError().body("Не удалось открыть файл для записи");
        }
        HttpResponse::Ok().json(users)
}

pub async fn echo(msg: web::Json<RegisterResponse>) -> impl Responder {
    
    HttpResponse::Ok().json(msg.into_inner())
}
