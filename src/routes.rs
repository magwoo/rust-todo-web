use actix_web::{get, HttpResponse};
use dioxus::prelude::*;
use dioxus_ssr::render_lazy as ssr;

#[get("/")]
pub async fn index() -> HttpResponse {
    let content = std::fs::read_to_string("./static/index.html").unwrap();
    HttpResponse::Ok().body(content)
}

#[get("/home")]
pub async fn home() -> HttpResponse {
    let content = rsx!(div {
        h1 {
            text_align: "center",
            "Todo.rs"
        }
    });
    HttpResponse::Ok().body(ssr(content))
}
