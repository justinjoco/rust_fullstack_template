use actix_web::{web, App, HttpServer, HttpRequest, Responder};

async fn greet(req: HttpRequest) -> impl Responder{
    format!("Hello world!")
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {

        // Start the Actix Web server
        HttpServer::new(|| {
            App::new()
                .route("/health_check", web::get().to(greet))
        })
        .bind("127.0.0.1:5000")?
        .run()
        .await
}
