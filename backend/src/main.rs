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
use r2d2_redis::RedisConnectionManager;
use r2d2::{Error, Pool};
#[get("/health_check")]
async fn health_check() -> impl Responder{
    info!("Health check ping");
    HttpResponse::Ok()
}

pub async fn establish_db_connection(database_url: &str) -> Result<PgPool, sqlx::Error> {
    let pool = PgPool::connect(database_url).await?;
    info!("Postgres connection success!");
    Ok(pool)
}

pub async fn establish_redis_connection(redis_url: &str) -> Result<Pool<RedisConnectionManager>, Error> {
    let manager = RedisConnectionManager::new(redis_url).unwrap();
    let pool = Pool::builder()
        .max_size(5)
        .build(manager)?;
    info!("Redis connection success!");
    Ok(pool)
}


#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
        env_logger::init();

        let database_url = "postgres://admin:password@postgres:5432/app_db";
        let redis_url = "redis://default:mypassword@cache:6379/0";

        let sql_pool = establish_db_connection(database_url).await.expect("bad postgres connection");
        let redis_pool = establish_redis_connection(redis_url).await.expect("bad redis connection");

        let book_repository = BookRepository::new(sql_pool, redis_pool);
        let _ : () = book_repository.seed_redis_cache().await.expect("could not seed");


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
