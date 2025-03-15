use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use log::info;
use uuid::Uuid;
use std::error::Error;

use crate::{model::book::{ Book, CreateBookRequest, UpdateBookRequest}, service::book_service::BookService};


#[get("/book/{id}")]
pub async fn get_book_by_id(path: web::Path<Uuid>, state: web::Data<BookService>) -> impl Responder{
    let id = path.into_inner();
    info!("Book by id: {}", id);
    let result: Result<Book, Box <dyn Error>> = state.get_ref().get_book_by_id(id).await;
    match result {
        Ok(val) =>   HttpResponse::Ok().json(val),
        Err(e) => HttpResponse::NotFound().json(e.to_string()),
    }
  
}


#[get("/books")]
pub async fn list_books(state: web::Data<BookService>) -> impl Responder{
    info!("List all books");
    let result: Result<Vec<Book>, Box <dyn Error>> = state.get_ref().list_books().await;
    match result {
        Ok(val) =>   HttpResponse::Ok().json(val),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
  
}

#[post("/book")]
pub async fn create_book(data: web::Json<CreateBookRequest>, state: web::Data<BookService>) -> impl Responder{
    info!("Create book: {}", data);
    let result: Result<Book, Box <dyn Error>>  = state.get_ref().create_book(data.into_inner()).await;
    match result {
        Ok(val) =>   HttpResponse::Ok().json(val),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
  
}

#[put("/book/{id}")]
pub async fn update_book_by_id(path: web::Path<Uuid>, data: web::Json<UpdateBookRequest>, state: web::Data<BookService>) -> impl Responder{
    let id = path.into_inner();
    info!("Update book by id: {}; update = {}", id, data);
    let result: Result<Book, Box <dyn Error>>  = state.get_ref().update_book_by_id(id, data.into_inner()).await;
    match result {
        Ok(val) =>   HttpResponse::Ok().json(val),
        Err(e) => HttpResponse::NotFound().json(e.to_string()),
    }
  
}

#[delete("/book/{id}")]
pub async fn delete_book_by_id(path: web::Path<Uuid>, state: web::Data<BookService>) -> impl Responder{
    let id = path.into_inner();
    info!("Delete book by id:  {}", id);
    let result : Result<bool, Box <dyn Error>>  = state.get_ref().delete_book_by_id(id).await;
    match result {
        Ok(val) =>   HttpResponse::Ok().json(val),
        Err(e) => HttpResponse::NotFound().json(e.to_string()),
    }
  
}
