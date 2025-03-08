mod controller;
mod service;
mod repository;
mod model;

use controller::book_controller::BookController;
use service::book_service::BookService;
use repository::book_repository::BookRepository;

use actix_web::{web, App, HttpServer, HttpResponse, Responder};

async fn health_check() -> impl Responder{
    HttpResponse::Ok()
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {

        let _book_repository = BookRepository::new();
        let _book_service = BookService::new();
        let _book_controller = BookController:: new();
        // Start the Actix Web server
        HttpServer::new(|| {
            App::new()
                .route("/health_check", web::get().to(health_check))
        })
        .bind("127.0.0.1:5000")?
        .run()
        .await
}
