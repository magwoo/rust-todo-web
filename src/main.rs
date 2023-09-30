use actix_web::{App, HttpServer};
use log::info;

pub mod database;
pub mod prelude;
mod routes;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    std::env::set_var("RUST_LOG", "INFO");
    env_logger::init();
    let server = HttpServer::new(|| {
        App::new()
            .service(routes::index)
            .service(routes::home)
            .service(routes::tasks)
            .service(routes::add_task)
            .service(routes::favicon)
    })
    .bind(("0.0.0.0", 7878))?
    .run();
    info!("Server started at http://127.0.0.1:7878");
    server.await
}
