use actix_web::{get, HttpResponse};

#[get("/")]
pub async fn index() -> HttpResponse {
    HttpResponse::Ok().finish()
}
