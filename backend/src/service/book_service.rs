//use crate::repository::book_repository::BookRepository;

/*
pub trait Service{
    pub fn list_all_books(&self);
    pub fn get_book_by_id(&self);
    pub fn delete_book_by_id(&self);
    pub fn update_book_by_id(&self);
    pub fn create_book(&self);
}
impl BookService{
    pub fn new() -> Self{
        
    }
}
impl Service for BookService{

}
*/

use crate::repository::book_repository::BookRepository;


pub struct BookService{
    _repository: BookRepository
}

impl BookService{
    pub fn new(repository: BookRepository) -> Self{
        BookService{_repository: repository}
    }
}
