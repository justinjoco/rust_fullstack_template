mod controller;
mod service;
mod repository;
mod model;

use controller::book_controller::BookController;
use diesel::PgConnection;
use diesel::r2d2::{Pool, ConnectionManager};
use service::book_service::BookService;
use repository::book_repository::BookRepository;
use log::info;
use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use env_logger;

async fn health_check() -> impl Responder{
    info!("Health check ping");
    HttpResponse::Ok()
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
        env_logger::init();

        let database_url = "postgres://admin:password@postgres:5432/app_db";
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool = Pool::builder().build(manager).expect("Failed to create pool.");

        let book_repository = BookRepository::new(pool);
        let book_service = BookService::new(book_repository);
        let _book_controller = BookController::new(book_service);
        // Start the Actix Web server
        HttpServer::new(|| {
            App::new()
                .route("/health_check", web::get().to(health_check))
        })
        .bind("0.0.0.0:5000")?
        .run()
        .await
}
