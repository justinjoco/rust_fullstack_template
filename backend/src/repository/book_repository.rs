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

use uuid::Uuid;
use sqlx::{PgPool, query, Row};
use crate::model::book::Book;

pub struct BookRepository{
    pool: PgPool, // The pool manages lifetime for us
}

impl BookRepository{
    pub fn new(  pool: PgPool  ) -> Self{
        BookRepository{pool: pool}
    }

   
    pub async fn get_book_by_id(&self, book_id: Uuid) -> Result<Book,  sqlx::Error>{
        let query_stmt = "SELECT (id, title, author, genre, description, rating, date_published) FROM users WHERE id = $1";
    
        let row = query(query_stmt)
            .bind(book_id)
            .fetch_one(&self.pool)
            .await?;
  

        Ok(Book{
            id: row.get("id"),
            title: row.get("title"),
            author: row.get("author"),
            genre: row.get("genre"),
            description: row.try_get("description").ok(),
            rating: row.try_get("rating").ok(),
            date_published: row.try_get("date_published").ok(),
        }
    )
    }


}