use actix_web::{get, post, web::Form, HttpResponse};
use dioxus::prelude::*;
use dioxus_ssr::render_lazy as ssr;
use serde::{Deserialize, Serialize};

use crate::prelude::*;

#[get("/")]
pub async fn index() -> HttpResponse {
    let content = include_str!("../static/index.html");
    HttpResponse::Ok().body(content)
}

#[get("/favicon")]
pub async fn favicon() -> Vec<u8> {
    let image = include_bytes!("../static/favicon.ico");
    image.to_vec()
}

#[get("/home")]
pub async fn home() -> HttpResponse {
    let form = rsx!(form {
        "hx-post": "/add-task",
        "hx-swap": "none",
        input {
            "type": "text",
            name: "title",
            background_color: "#363636"
        },
        button {
            background_color: "#363636",
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
            justify_content: "Center",
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
            task_builder(format!("{}. {}", i + 1, task.title))
        }
    });
    HttpResponse::Ok().body(ssr(content))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskForm {
    title: String,
}

#[post("/add-task")]
pub async fn add_task(task: Form<TaskForm>) -> HttpResponse {
    if let Err(e) = Task::new(&task.title).insert_db() {
        return HttpResponse::InternalServerError().body(e.to_string());
    }
    HttpResponse::Ok().finish()
}
