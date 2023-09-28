use actix_web::{get, App, HttpResponse, HttpServer};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let server = HttpServer::new(|| App::new().service(index))
        .bind(("0.0.0.0", 80))?
        .run();
    server.await
}

#[get("/")]
async fn index() -> HttpResponse {
    HttpResponse::Ok().finish()
}
