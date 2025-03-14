//use crate::service::book_service::BookService;

/*
pub trait Controller {
     fn list_all_books(&self);
     fn get_book_by_id(&self);
     fn delete_book_by_id(&self);
     fn update_book_by_id(&self);
     fn create_book(&self);
}
impl BookController{
    pub fn new() -> Self{

    }
}


*/
use actix_web::{ App, web, HttpServer, HttpResponse, Responder, get, post, delete, put};
use log::{info, logger};
use uuid::Uuid;

use crate::{model::book::{self, Book, CreateBookRequest, UpdateBookRequest}, service::book_service::BookService};


#[get("/book/{book_id}")]
pub async fn get_book_by_id(path: web::Path<Uuid>, state: web::Data<BookService>) -> impl Responder{
    let book_id = path.into_inner();
    info!("Book by id: {}", book_id);
    HttpResponse::Ok()
}


#[get("/books")]
pub async fn list_books(state: web::Data<BookService>) -> impl Responder{
    info!("List all books");
    HttpResponse::Ok()
}

#[post("/book")]
pub async fn create_book(data: web::Json<CreateBookRequest>, state: web::Data<BookService>) -> impl Responder{
    info!("Create book: {}", data);
    HttpResponse::Ok()
}

#[put("/book/{book_id}")]
pub async fn update_book_by_id(path: web::Path<Uuid>, data: web::Json<UpdateBookRequest>, state: web::Data<BookService>) -> impl Responder{
    let book_id = path.into_inner();
    info!("Update book by id: {}; update = {}", book_id, data);
    HttpResponse::Ok()
}

#[delete("/book/{book_id}")]
pub async fn delete_book_by_id(path: web::Path<Uuid>, state: web::Data<BookService>) -> impl Responder{
    let book_id = path.into_inner();
    info!("Delete book by id:  {}", book_id);
    HttpResponse::Ok()
}
