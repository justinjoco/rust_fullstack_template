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

use crate::service::book_service::BookService;


pub struct BookController{
    _service: BookService
}

impl BookController{
    pub fn new(service: BookService) -> Self{
        BookController{_service: service}
    }

    
}