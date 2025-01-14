use actix_web::{middleware, web, App, HttpResponse, HttpServer};
use env_logger;
use log;
mod handlers;
mod models;
use crate::handlers::auth_handlers::register; 

#[actix_web::main]
async fn main() -> std::io::Result<()> { 
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    log::info!("starting HTTP server at http://localhost:8080");
        
    HttpServer::new(move || {
         App::new()
            .wrap(middleware::Logger::default().log_target("http_log"))
            .route("/", web::get().to(|| async { HttpResponse::Ok().body("Hello") }))
            .route("/register", web::post().to(register))
    })
    .bind("127.0.0.1:8080")? 
    .run()
    .await
}
