use actix_web::{App, HttpServer};

pub mod database;
pub mod prelude;
mod routes;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let server = HttpServer::new(|| App::new().service(routes::index))
        .bind(("0.0.0.0", 80))?
        .run();
    server.await
}
