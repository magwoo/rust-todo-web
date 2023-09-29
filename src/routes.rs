use actix_web::{get, post, HttpResponse};
use dioxus::prelude::*;
use dioxus_ssr::render_lazy as ssr;

use crate::prelude::*;

#[get("/")]
pub async fn index() -> HttpResponse {
    let content = std::fs::read_to_string("./static/index.html").unwrap();
    HttpResponse::Ok().body(content)
}

#[get("/home")]
pub async fn home() -> HttpResponse {
    let form = rsx!(form {
        "hx-post": "/add-task",
        input {
            "type": "text",
            id: "title",
            background_color: "#363636"
        },
        button {
            "add"
        }
    });
    let content = rsx!(div {
        h1 {
            text_align: "center",
            margin: "2rem",
            "Todo.rs"
        },
        div {
            display: "Flex",
            form
        }
    });
    HttpResponse::Ok().body(ssr(content))
}

#[get("/tasks")]
pub async fn tasks() -> HttpResponse {
    let tasks = Task::get_all_db().unwrap();
    let task_builder = |title| {
        rsx!(div {
            margin_bottom: "1rem",
            "{title}"
        })
    };
    let content = rsx!(div {
        margin: "2rem",
        for (i, task) in tasks.iter().enumerate() {
            task_builder(format!("{}. {}", i, task.title))
        }
    });
    HttpResponse::Ok().body(ssr(content))
}

#[post("/add-task")]
pub async fn add_task() -> HttpResponse {
    HttpResponse::Ok().finish()
}
