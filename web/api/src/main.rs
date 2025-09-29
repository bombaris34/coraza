mod action_logs;
mod activation_keys;
mod auth;
mod db;
mod internal;
mod logs;
mod models;
mod products;
mod stats;
mod users;

use actix_cors::Cors;
use actix_files::Files;
use actix_web::{get, http, middleware::Logger, web, App, HttpResponse, HttpServer, Responder};
use log::info;

#[get("/")]
async fn hello() -> impl Responder {
    info!("GET /");
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_web=debug,api=debug");
    env_logger::init();

    let _db_conn = db::establish_connection().await;

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("https://coraza.clothing")
            .allowed_origin("https://panel.coraza.clothing")
            .allowed_origin(
                &std::env::var("FRONTEND_URL")
                    .unwrap_or_else(|_| "http://localhost:8080".to_string()),
            )
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);

        App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .service(Files::new("/uploads", "uploads").show_files_listing())
            .service(hello)
            .service(
                web::scope("/auth")
                    .service(auth::routes::login)
                    .service(auth::routes::register)
                    .service(auth::routes::validate_token),
            )
            .service(web::scope("/products").configure(products::routes::init_routes))
            .service(
                web::scope("/users")
                    .wrap(auth::middleware::Authentication)
                    .configure(users::routes::init_routes),
            )
            .service(
                web::scope("/internal")
                    .wrap(auth::internal_middleware::InternalAuthentication)
                    .configure(internal::routes::init_routes),
            )
            .service(
                web::scope("/admin")
                    .wrap(auth::middleware::Authentication)
                    .configure(products::routes::init_admin_routes)
                    .configure(logs::init_routes)
                    .service(activation_keys::routes::create_activation_key)
                    .service(activation_keys::routes::get_activation_keys)
                    .service(activation_keys::routes::delete_activation_key)
                    .service(activation_keys::routes::replace_activation_key),
            )
            .service(
                web::scope("/stats")
                    .wrap(auth::middleware::Authentication)
                    .service(stats::routes::get_registration_stats),
            )
    })
    .bind(&std::env::var("SERVER_BIND_ADDRESS").unwrap_or_else(|_| "127.0.0.1:8081".to_string()))?
    .run()
    .await
}
