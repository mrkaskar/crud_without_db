use std::sync::Mutex;

use actix_cors::Cors;
use actix_web::HttpResponse;
use actix_web::{http::header, web, App, HttpServer, Responder};

mod db;
mod model;

use db::Db;
use model::Task;

struct AppState {
    db: Mutex<Db>,
}

const DB_NAME: &str = "db.json";

async fn create_task_handler(
    app_state: web::Data<AppState>,
    task: web::Json<Task>,
) -> impl Responder {
    let mut db = app_state.db.lock().unwrap();
    db.insert_task(task.into_inner());
    let _ = db.save_to_file(DB_NAME);
    HttpResponse::Ok().finish()
}

async fn get_tasks_handler(app_state: web::Data<AppState>) -> impl Responder {
    let db = app_state.db.lock().unwrap();
    let tasks = db.get_tasks();
    HttpResponse::Ok().json(tasks)
}

async fn get_task_handler(
    app_state: web::Data<AppState>,
    task_id: web::Path<u64>,
) -> impl Responder {
    let db = app_state.db.lock().unwrap();
    match db.get_task(&task_id) {
        Some(task) => HttpResponse::Ok().json(task),
        None => HttpResponse::NotFound().finish(),
    }
}

async fn update_task_handler(
    app_state: web::Data<AppState>,
    task_id: web::Path<u64>,
    updated_task: web::Json<Task>,
) -> impl Responder {
    let mut db = app_state.db.lock().unwrap();
    db.update_task(*task_id, updated_task.into_inner());
    let _ = db.save_to_file(DB_NAME);
    HttpResponse::Ok().finish()
}

async fn delete_task_handler(
    app_state: web::Data<AppState>,
    task_id: web::Path<u64>,
) -> impl Responder {
    let mut db = app_state.db.lock().unwrap();
    db.delete_task(*task_id);
    let _ = db.save_to_file(DB_NAME);
    HttpResponse::Ok().finish()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = match Db::load_from_file(DB_NAME) {
        Ok(db) => db,
        Err(e) => {
            eprintln!("Failed to load database: {}", e);
            Db::new()
        }
    };
    let data = web::Data::new(AppState { db: Mutex::new(db) });

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::permissive()
                    .allowed_origin_fn(|origin, _req_head| {
                        origin.as_bytes().starts_with(b"http://localhost") || origin.is_empty()
                    })
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::CONTENT_TYPE])
                    .supports_credentials()
                    .max_age(3600),
            )
            .app_data(data.clone())
            .route("/tasks", web::post().to(create_task_handler))
            .route("/tasks", web::get().to(get_tasks_handler))
            .route("/tasks/{id}", web::get().to(get_task_handler))
            .route("/tasks/{id}", web::put().to(update_task_handler))
            .route("/tasks/{id}", web::delete().to(delete_task_handler))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
