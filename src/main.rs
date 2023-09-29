use actix_web::{App, HttpServer};

pub mod database;
pub mod prelude;
mod routes;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .service(routes::index)
            .service(routes::home)
            .service(routes::tasks)
            .service(routes::add_task)
    })
    .bind(("0.0.0.0", 7878))?
    .run();
    server.await
}
