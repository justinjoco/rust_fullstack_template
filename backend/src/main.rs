#![allow(dead_code)]

mod controller;
mod service;
mod repository;
mod model;

use controller::book_controller::{get_book_by_id, list_books, delete_book_by_id, update_book_by_id, create_book};
use service::book_service::BookService;
use repository::book_repository::BookRepository;
use log::info;
use actix_web::{ App, HttpServer, HttpResponse, Responder, get, web};
use env_logger;
use sqlx::PgPool;
use actix_cors::Cors;

#[get("/health_check")]
async fn health_check() -> impl Responder{
    info!("Health check ping");
    HttpResponse::Ok()
}

pub async fn establish_connection(database_url: &str) -> Result<PgPool, sqlx::Error> {
    let pool = PgPool::connect(database_url).await?;
    Ok(pool)
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
        env_logger::init();

        let database_url = "postgres://admin:password@postgres:5432/app_db";
        let pool = establish_connection(database_url).await.expect("bad postgres connection");

        let book_repository = BookRepository::new(pool);
        let book_service = BookService::new(book_repository);
        // Start the Actix Web server
        HttpServer::new(move || App::new()
            .wrap(Cors::default()
                .allow_any_origin()
                .send_wildcard()
                .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])  // Allowed methods
                .allowed_headers(vec!["Content-Type", "Authorization", "Origin"])  // Allowed headers
                .max_age(3600))
            .app_data(web::Data::new(book_service.clone()))
            .service(health_check)
            .service(get_book_by_id)
            .service(create_book)
            .service(update_book_by_id)
            .service(delete_book_by_id)
            .service(list_books)

        )
        .bind("0.0.0.0:5000")?
        .run()
        .await
}
