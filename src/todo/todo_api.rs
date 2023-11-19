use actix_web::{HttpResponse, Responder, web};
use actix_web::web::Json;
use serde::Deserialize;

use crate::todo::todo::Todo;
use crate::todo_service;

type TodoId = u64;

#[derive(Deserialize)]
pub struct TodoParams {
    page: Option<u64>,
    limit: Option<u64>,
}

pub async fn get_todo_list() -> impl Responder {
    let result = todo_service!().get_todo_list().await;
    match result {
        Ok(list) => { HttpResponse::Ok().json(list) }
        Err(err) => { HttpResponse::BadRequest().json(err) }
    }
}

pub async fn get_todo_by_id(path: web::Path<TodoId>) -> impl Responder {
    let result = todo_service!().get_todo_by_id(path.into_inner()).await;
    match result {
        Ok(todo) => {
            match todo {
                None => { HttpResponse::NotFound().finish() }
                Some(data) => { HttpResponse::Ok().json(data) }
            }
        }
        Err(err) => { HttpResponse::BadRequest().json(err) }
    }
}

pub async fn get_page_todo(params: web::Query<TodoParams>) -> impl Responder {
    let page = params.page.to_owned().unwrap_or(0);
    let limit = params.limit.unwrap_or(20);
    let result = todo_service!().get_todo_page(page, limit).await;
    match result {
        Ok(page) => { HttpResponse::Ok().json(page) }
        Err(err) => { HttpResponse::BadRequest().json(err) }
    }
}

pub async fn create_todo(todo: Json<Todo>) -> impl Responder {
    let result = todo_service!().insert_todo(todo.0).await;
    match result {
        Ok(todo) => { HttpResponse::Ok().json(todo) }
        Err(err) => { HttpResponse::BadRequest().json(err) }
    }
}

pub async fn update_todo(todo: Json<Todo>) -> impl Responder {
    let v = todo.0;
    let result = todo_service!().update_todo(v).await;
    match result {
        Ok(_) => { HttpResponse::Ok().finish() }
        Err(err) => { HttpResponse::BadRequest().json(err) }
    }
}

pub async fn delete_todo(path: web::Path<TodoId>) -> impl Responder {
    let result = todo_service!().delete_todo_by_id(path.into_inner()).await;
    match result {
        Ok(_) => { HttpResponse::Ok().finish() }
        Err(err) => { HttpResponse::BadRequest().json(err) }
    }
}