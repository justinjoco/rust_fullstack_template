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
use sqlx::{PgPool, query, query_as, Row};
use crate::model::book::{Book, CreateBookRequest, UpdateBookRequest};

#[derive(Clone)]
pub struct BookRepository{
    pool: PgPool, 
}

impl BookRepository{
    pub fn new(  pool: PgPool  ) -> Self{
        BookRepository{pool: pool}
    }

   
    pub async fn get_book_by_id(&self, book_id: Uuid) -> Result<Book,  sqlx::Error>{
        let query_stmt = "SELECT * FROM book WHERE id = $1";
    
        let book = query_as::<_,Book>(query_stmt)
            .bind(book_id)
            .fetch_one(&self.pool)
            .await?;
  

        Ok(book)
    }

    pub async fn list_books(&self) -> Result<Vec<Book>,  sqlx::Error>{
        let query_stmt = "SELECT * FROM book";
    
        let books = query_as::<_, Book>(query_stmt)
            .fetch_all(&self.pool)
            .await?;
  

        Ok(books)
    }

    
    pub async fn delete_book_by_id(&self, id: Uuid) -> Result<bool,  sqlx::Error>{
        let query_stmt = "DELETE FROM book WHERE id = $1";
        let result = query(query_stmt).bind(id).execute(&self.pool).await?;
        Ok(result.rows_affected() > 0)
    }

    pub async fn save(&self, new_book: Book) -> Result<Book, sqlx::Error>{
        let query_stmt: &str = "
        INSERT INTO book (id, title, author, genre, description, rating, date_published) 
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        ON CONFLICT (id)
        DO UPDATE
        SET title = EXCLUDED.title, author = EXCLUDED.author, genre = EXCLUDED.genre, description = EXCLUDED.description, rating = EXCLUDED.rating, date_published = EXCLUDED.date_published
        RETURNING id, title, author, genre, description, rating, date_published
        ";
        let book = query_as::<_,Book>(query_stmt)
            .bind(new_book.id)
            .bind(new_book.title)
            .bind(new_book.author)
            .bind(new_book.genre)
            .bind(new_book.description)
            .bind(new_book.rating)
            .bind(new_book.date_published)
            .fetch_one(&self.pool).await?;
        Ok(book)
    }


}