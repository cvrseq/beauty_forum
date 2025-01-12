use actix_web::{ HttpResponse, Responder};




pub async fn login() -> impl Responder {
    HttpResponse::Ok().body("User authenticated")
}

pub async fn greetings() -> impl Responder { 
    HttpResponse::Ok().body("Welcome to the club, body and ladies and gentlemen!!!")
}


