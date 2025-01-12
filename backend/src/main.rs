

use actix_web::{middleware, web, App, HttpResponse, HttpServer};
use {env_logger, log};


mod handlers { 
    pub mod auth_handlers;
}

mod models { 
    pub mod user;
}

mod db { 
    pub mod base;
}

mod routes { 
    pub mod auth;
}

mod utils { 
    pub mod sms;
}

#[actix_web::main]
async fn main() -> std::io::Result<()> { 
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    log::info!("starting HTTP server at http://localhost:8080");
        
    
    HttpServer::new(||
         App::new()
            .wrap(middleware::Logger::default().log_target("http_log"))
            .route("/login", web::get().to(handlers::auth_handlers::login))
            .route("/", web::get().to(handlers::auth_handlers::greetings)))
            
            
        .bind("127.0.0.1:8080")? 
        .run()
        .await
}

