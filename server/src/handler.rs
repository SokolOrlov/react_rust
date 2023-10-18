
use actix_web::{web, HttpResponse, Responder, get, post};
use sqlx::Row;
use serde_json::json;

use crate::AppState;
use crate::model::Todo;
use crate::schema::{CreateTodo, UpdateTodo};


pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api")
        .service(health_checker_handler)
        .service(get_all_handler)
        .service(create_handler)
        .service(delete_handler)
        .service(update_handler);

    conf.service(scope);
}

#[get("/all")]
pub async fn get_all_handler(data: web::Data<AppState>) -> impl Responder {
    let select_query = sqlx::query_as::<_, Todo>("SELECT id, name, done FROM todos");
    let tickets: Vec<Todo> = select_query.fetch_all(&data.db).await.unwrap();

    let json_response = serde_json::json!({
        "status": "success",
        "results": tickets.len(),
        "notes": tickets
    });
    HttpResponse::Ok().json(json_response)
}

#[post("/create")]
pub async fn create_handler(body: web::Json<CreateTodo>, data: web::Data<AppState>) -> impl Responder {
    let query_result= sqlx::query("INSERT INTO todos (name, done) VALUES ($1, $2) returning id")
    .bind(&body.name)
    .bind(body.done)
    .fetch_one(&data.db).await;

    let result = match query_result {
        Ok(id)=> serde_json::json!({
            "status": "success",    
            "id": id.get::<i64, &str>("id")
        }),
        Err(e)=>serde_json::json!({
            "status": "error",
            "message": e.to_string()
        })
    };

    HttpResponse::Ok().json(result)
}

#[get("/delete/{id}")]
pub async fn delete_handler(path: web::Path<i64>, data: web::Data<AppState>) -> impl Responder {
    let query_result= sqlx::query("DELETE FROM todos WHERE id = $1")
    .bind(path.into_inner())
    .execute(&data.db)
    .await;

    let result = match query_result {
        Ok(_)=> serde_json::json!({
            "status": "success"
        }),
        Err(e)=>serde_json::json!({
            "status": "error",
            "message": e.to_string()
        })
    };

    HttpResponse::Ok().json(result)
}

#[post("/update")]
pub async fn update_handler(body: web::Json<UpdateTodo>, data: web::Data<AppState>) -> impl Responder {
    let query_result= sqlx::query("UPDATE todos SET name = $1, done = $2 WHERE id = $3")
    .bind(&body.name)
    .bind(body.done)
    .bind(&body.id)
    .execute(&data.db).await;

    let result = match query_result {
        Ok(_)=> serde_json::json!({
            "status": "success"
        }),
        Err(e)=>serde_json::json!({
            "status": "error",
            "message": e.to_string()
        })
    };

    HttpResponse::Ok().json(result)
}

#[get("/healthChecker")]
async fn health_checker_handler() -> impl Responder {
    HttpResponse::Ok().json(json!({"status":"success", "message": "API runing."}))
}