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

use crate::{model::book::{ Book, CreateBookRequest, UpdateBookRequest}, service::book_service::BookService};


#[get("/book/{id}")]
pub async fn get_book_by_id(path: web::Path<Uuid>, state: web::Data<BookService>) -> impl Responder{
    let id = path.into_inner();
    info!("Book by id: {}", id);
    let result: Book = state.get_ref().get_book_by_id(id).await;
    HttpResponse::Ok().json(result)
}


#[get("/books")]
pub async fn list_books(state: web::Data<BookService>) -> impl Responder{
    info!("List all books");
    let result: Vec<Book> = state.get_ref().list_books().await;
    HttpResponse::Ok().json(result)
}

#[post("/book")]
pub async fn create_book(data: web::Json<CreateBookRequest>, state: web::Data<BookService>) -> impl Responder{
    info!("Create book: {}", data);
    let result: Book = state.get_ref().create_book(data.into_inner()).await;
    HttpResponse::Ok().json(result)
}

#[put("/book/{id}")]
pub async fn update_book_by_id(path: web::Path<Uuid>, data: web::Json<UpdateBookRequest>, state: web::Data<BookService>) -> impl Responder{
    let id = path.into_inner();
    info!("Update book by id: {}; update = {}", id, data);
    let result: Book = state.get_ref().update_book_by_id(id, data.into_inner()).await;
    HttpResponse::Ok().json(result)
}

#[delete("/book/{id}")]
pub async fn delete_book_by_id(path: web::Path<Uuid>, state: web::Data<BookService>) -> impl Responder{
    let id = path.into_inner();
    info!("Delete book by id:  {}", id);
    let result = state.get_ref().delete_book_by_id(id).await;
    HttpResponse::Ok().json(result)
}
