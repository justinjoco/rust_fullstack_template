//use crate::model::book::Book;
/*
pub trait Repository{
    pub fn list_all_books(&self);
    pub fn get_book_by_id(&self);
    pub fn delete_book_by_id(&self);
    pub fn update_book_by_id(&self);
    pub fn create_book(&self);
}

impl BookRepository{
    pub fn new() -> Self{
        
    }
}

impl Repository for BookRepository{
    
}
*/

use diesel::PgConnection;
use diesel::r2d2::{ConnectionManager,Pool};

pub struct BookRepository{
    _pool: Pool<ConnectionManager<PgConnection>>, // The pool manages lifetime for us
}

impl BookRepository{
    pub fn new(  pool: Pool<ConnectionManager<PgConnection>>    ) -> Self{
        BookRepository{_pool: pool}
    }
}