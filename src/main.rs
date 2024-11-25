#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde;

use std::fs as stdfs;
use std::time::Duration;

use actix_cors::Cors;
use actix_files as fs;
use actix_web::{http::header, middleware::Logger, web, App, HttpServer};
use controller::{file_controller, role_controller, root_controller, user_controller};
use dotenv::dotenv;
use sqlx::postgres::{PgPool, PgPoolOptions};
use utils::{swagger_docs::ApiDoc, web_socket::ws_index};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod config;
mod controller;
mod dto;
mod entity;
mod error;
mod query;
mod service;
mod utils;

pub struct PgState {
    pub db: PgPool,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let origins = std::env::var("ORIGINS").expect("ORIGINS must be set");
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            println!("Connection to the database is successful !");
            pool
        }
        Err(err) => {
            println!("Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };

    let dir_creator_image = stdfs::create_dir("./upload/image");
    match dir_creator_image {
        Ok(()) => {}
        Err(_err) => {
            println!("Skip create directory /upload/image, directory is exist.")
        }
    }

    let dir_creator_file = stdfs::create_dir("./upload/file");
    match dir_creator_file {
        Ok(()) => {}
        Err(_err) => {
            println!("Skip create directory /upload/file, directory is exist.")
        }
    }
    let openapi = ApiDoc::openapi();
    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin(&origins)
            .allowed_methods(vec!["GET", "POST", "PATCH", "DELETE"])
            .allowed_headers(vec![
                header::CONTENT_TYPE,
                header::AUTHORIZATION,
                header::ACCEPT,
            ])
            .supports_credentials();
        App::new()
            .app_data(web::Data::new(PgState { db: pool.clone() }))
            .route("/ws/", web::get().to(ws_index))
            .service(
                web::scope("/api")
                    .configure(root_controller::root_route_config)
                    .configure(file_controller::file_route_config)
                    .configure(role_controller::role_route_config)
                    .configure(user_controller::user_route_config),
            )
            .service(fs::Files::new("/upload", "./upload").use_last_modified(true))
            .service(SwaggerUi::new("/docs/{_:.*}").url("/api-docs/openapi.json", openapi.clone()))
            .wrap(cors)
            .wrap(Logger::default())
    })
    .client_request_timeout(Duration::from_secs(15))
    .workers(2)
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
