#[macro_use]
extern crate diesel;

use actix_files::Files;
use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::{middleware, web, App, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

mod errors;
mod handlers;
mod models;
mod schema;
mod services;
mod utils;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    std::env::set_var( "RUST_LOG", "api_server=debug,actix_web=info,actix_server=info", );
    env_logger::init();
    
    // create db connection pool
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<MysqlConnection>::new(database_url);
    let pool: models::Pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");
    let domain: String =
        std::env::var("DOMAIN").unwrap_or_else(|_| "localhost".to_string());
    let port: String = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let bind_address = format!("{}:{}", &domain, &port);

    // Start http server
    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .data(web::JsonConfig::default().limit(4096))
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(utils::SECRET_KEY.as_bytes())
                    .name("auth")
                    .path("/")
                    .domain(domain.as_str())
                    .max_age_time(chrono::Duration::days(1))
                    .secure(false),
            ))
            .wrap(middleware::Logger::default())
            .service(
                web::scope("/api")
                    .service(
                        web::resource("/invitation")
                            .route(web::post().to(handlers::invitation::post_invitation)),
                    )
                    .service(
                        web::resource("/register/{invitation_id}")
                            .route(web::post().to(handlers::register::register_user)),
                    )
                    .service(
                        web::resource("/auth")
                            .route(web::post().to(handlers::auth::login))
                            .route(web::delete().to(handlers::auth::logout))
                            .route(web::get().to(handlers::auth::get_me)),
                    ),
            )
            .default_service(Files::new("", "../frontend/public"))
    })
    .bind(&bind_address)?
    .run()
    .await
}
