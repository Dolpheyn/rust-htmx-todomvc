mod components;
mod error;

use std::{
    collections::HashMap,
    sync::{atomic::AtomicUsize, Mutex},
};

use crate::components::todo_item::TodoItem;
use crate::error::MyError;

use actix_web::{get, post, put, web, App, HttpRequest, HttpResponse, HttpServer};
use leptos::*;

#[get("/")]
async fn index(_req: HttpRequest, data: web::Data<AppState>) -> HttpResponse {
    let todos = data.todos.lock().unwrap();
    let mut todos = todos.values().map(|t| t.clone()).collect::<Vec<_>>();
    todos.sort_by(|a, b| a.id.cmp(&b.id));

    let html = leptos::ssr::render_to_string(|cx| {
        view! {cx,
            <head>
                <script src="https://unpkg.com/htmx.org@1.9.2" integrity="sha384-L6OqL9pRWyyFU3+/bjdSri+iIphTN/bvYyM37tICVyOJkWZLpP2vGn6VUEXgzg6h" crossorigin="anonymous"></script>
            </head>
            <body>
                <button
                    hx-post="/todos"
                    hx-trigger="click"
                    hx-target="#todo-list"
                    hx-swap="beforeend"
                >
                    {"Add todo"}
                </button>
                <ul id="todo-list">
                <For
                    // a function that returns the items we're iterating over; a signal is fine
                    each=move || todos.clone()
                    // a unique key for each item
                    key=|t| t.id
                    // renders each item to a view
                    view=move |cx, t: TodoItem| {
                        view! {
                            cx,
                            <TodoItem id={t.id} text={t.text.clone()} completed={t.completed} />
                        }
                    }
                />
                </ul>
            </body>
        }
    });

    return HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html);
}

#[post("/todos")]
async fn create_todo(
    _req: HttpRequest,
    data: web::Data<AppState>,
) -> Result<HttpResponse, MyError> {
    let id = data
        .counter
        .fetch_add(1, std::sync::atomic::Ordering::Relaxed);

    let new_todo = TodoItem {
        id,
        completed: false,
        text: "New todo".to_string(),
    };
    let mut todos = data.todos.lock().unwrap();
    todos.insert(id, new_todo.clone());

    let html = leptos::ssr::render_to_string(move |cx| {
        view! {cx,
            <TodoItem
                id={new_todo.id}
                text={new_todo.text.clone()}
                completed={new_todo.completed}
            />
        }
    });

    return Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html));
}

#[put("/todos/{id}")]
async fn toggle_todo(
    path: web::Path<usize>,
    data: web::Data<AppState>,
) -> Result<HttpResponse, MyError> {
    let id = path.into_inner();

    let mut todos = data.todos.lock().unwrap();
    let updated_todo = todos.get_mut(&id).unwrap();
    updated_todo.toggle_completed();

    let updated_todo = updated_todo.clone();
    let html = leptos::ssr::render_to_string(move |cx| {
        view! {cx,
            <TodoItem
                id={updated_todo.id}
                text={updated_todo.text.clone()}
                completed={updated_todo.completed}
            />
        }
    });

    return Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html));
}

#[derive(Clone)]
struct TodoItem {
    id: usize,
    completed: bool,
    text: String,
}

impl TodoItem {
    fn toggle_completed(&mut self) {
        self.completed = !self.completed;
    }
}

struct AppState {
    pub counter: AtomicUsize,
    pub todos: Mutex<HashMap<usize, TodoItem>>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(AppState {
                counter: AtomicUsize::new(1),
                todos: Mutex::new(HashMap::new()),
            }))
            .service(index)
            .service(create_todo)
            .service(toggle_todo)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
