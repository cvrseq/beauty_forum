use actix_web::{web, HttpResponse, Responder};
use serde_json;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use serde::Deserialize; 
use crate::models::user::User;
use rand::{thread_rng, Rng};

fn generate_confirmation_code() -> String {
    let code = thread_rng().gen_range(1000..=9999);
    code.to_string()
}

fn determine_provider(phone: &str) -> String {
    if phone.starts_with("+7") {
        "Beeline".to_string()
    } else if phone.starts_with("+375") {
        "A1".to_string()
    } else {
        "Unknown".to_string()
    }
}

pub async fn register(data: web::Json<UserRegisterInput>) -> impl Responder {
    let path = "users.json"; 
    let input = data.into_inner();

    let mut users: Vec<User> = Vec::new(); 
    if let Ok(mut file) = File::open(path) { 
        let mut content = String::new();
        if file.read_to_string(&mut content).is_ok() && !content.trim().is_empty() { 
            if let Ok(parsed) = serde_json::from_str::<Vec<User>>(&content) { 
                users = parsed;
            }
        }
    }
    
    if users.iter().any(|user| user.phone == input.phone) { 
        return HttpResponse::BadRequest()
            .body("Этот номер уже зарегистрирован");
    }
    
    let confirmation_code = generate_confirmation_code();
    
    let provider = determine_provider(&input.phone);
    
    let new_user = User {
        phone: input.phone,
        provider,
        confirmation_code: confirmation_code.clone(),
        confirmed: false,
    };

    let new_user_cloned = new_user.clone();

    users.push(new_user);

    if let Ok(mut file) = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)
    {
        let new_content = serde_json::to_string_pretty(&users)
            .unwrap_or_else(|_| "[]".to_string());
        if let Err(e) = file.write_all(new_content.as_bytes()) {
            return HttpResponse::InternalServerError().body(format!("Ошибка записи файла: {}", e));
        }
    } else {
        return HttpResponse::InternalServerError().body("Не удалось открыть файл для записи");
    }
    
    // (Опционально) Отправка SMS с кодом подтверждения может вызываться здесь,
    // используя реализацию send_sms из модуля utils::sms.
    // Например:
    // if let Err(e) = utils::sms::send_sms(&new_user.phone, &format!("Ваш код подтверждения: {}", confirmation_code)).await {
    //      return HttpResponse::InternalServerError().body(format!("Ошибка отправки SMS: {}", e));
    // }
    // Возвращаем ответ с сообщением
    HttpResponse::Ok().json(serde_json::json!({
        "message": "Код подтверждения отправлен",
        "phone": new_user_cloned.phone,
        "provider": new_user_cloned.provider
    }))
}

#[derive(Deserialize)]
pub struct UserRegisterInput {
    pub phone: String,
}
