use actix_web::{web, App, HttpServer, HttpResponse, Responder};

async fn health_check() -> impl Responder{
    HttpResponse::Ok()
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {

        // Start the Actix Web server
        HttpServer::new(|| {
            App::new()
                .route("/health_check", web::get().to(health_check))
        })
        .bind("127.0.0.1:5000")?
        .run()
        .await
}
