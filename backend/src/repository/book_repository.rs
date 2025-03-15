use log::info;
use uuid::Uuid;
use sqlx::{PgPool, query, query_as};
use crate::model::book::Book;
use r2d2::Pool;
use r2d2_redis::{redis::{Commands, RedisError}, RedisConnectionManager};
use std::error::Error;

#[derive(Clone)]
pub struct BookRepository{
    sql_pool: PgPool, 
    redis_pool: Pool<RedisConnectionManager>
}

impl BookRepository{
    pub fn new(  sql_pool: PgPool, redis_pool: Pool<RedisConnectionManager>  ) -> Self{
        BookRepository{sql_pool, redis_pool}
    }

    pub async fn seed_redis_cache(&self) -> Result<(), Box<dyn Error>>{
        let books = self.list_books_db().await?;
        let mut conn = self.redis_pool.get()?;
        for book in books {
            let key = format!("book:{}",book.id);
            let  res = serde_json::to_string(&book);
            if let Ok(value) = res {
                let _: Result<(), RedisError> = conn.set(key, value);
            }
        }
        info!("Redis cache seeded!");
        Ok(())
    }

   
    pub async fn get_book_by_id(&self, book_id: Uuid) -> Result<Book,  Box <dyn Error>>{
        let mut conn = self.redis_pool.get()?;
        let key = format!("book:{}", book_id);
        let result: Result<String, RedisError>  = conn.get(key);
        match result {
            Ok(value) => Ok(serde_json::from_str(&value)?),
            Err(_) =>  self.get_book_by_id_db(book_id).await,
        }       
    }

    async fn get_book_by_id_db(&self, book_id: Uuid) -> Result<Book,  Box <dyn Error>>{
        let query_stmt = "SELECT * FROM book WHERE id = $1";
    
        let book = query_as::<_,Book>(query_stmt)
            .bind(book_id)
            .fetch_one(&self.sql_pool)
            .await?;
  

        Ok(book)
    }

    pub async fn list_books(&self) -> Result<Vec<Book>,  Box<dyn Error>>{
        let mut conn = self.redis_pool.get()?;
        let key_result : Result<Vec<String>, RedisError> = conn.keys("book:*");
        if let Ok(keys) = key_result {
            let mut cached : Vec<Book> = Vec::new();
            for key in keys {
                let result: Result<String, RedisError>= conn.get(key);
                if let Ok(value) = result {
                    let book :Book = serde_json::from_str(&value)?;
                    cached.push(book);
                }
            }
            return Ok(cached);
        }
        self.list_books_db().await
    }

     async fn list_books_db(&self) -> Result<Vec<Book>,  Box<dyn Error>>{
        let query_stmt = "SELECT * FROM book";
    
        let books = query_as::<_, Book>(query_stmt)
            .fetch_all(&self.sql_pool)
            .await?;
  

        Ok(books)
    }

    
    pub async fn delete_book_by_id(&self, id: Uuid) -> Result<bool, Box <dyn Error>>{
        let mut conn = self.redis_pool.get()?;
        let key = format!("book:{}", id);
        let _: Result<String, RedisError> = conn.del(key);

        self.delete_book_by_id_db(id).await
    }

    async fn delete_book_by_id_db(&self, id: Uuid) -> Result<bool, Box <dyn Error>>{
        let query_stmt = "DELETE FROM book WHERE id = $1";
        let result = query(query_stmt).bind(id).execute(&self.sql_pool).await?;
        Ok(result.rows_affected() > 0)
    }


    pub async fn save(&self, new_book: Book) -> Result<Book, Box <dyn Error>>{
        let book_for_db = new_book.clone();
        self.save_db(book_for_db).await?;

        let mut conn = self.redis_pool.get()?;
        let key = format!("book:{}",new_book.id);
        let  res = serde_json::to_string(&new_book);
        if let Ok(value) = res {
            let _: Result<(), RedisError> = conn.set(key, value);
        }
        Ok(new_book)
    }

    
    async fn save_db(&self, new_book: Book) -> Result<Book, Box <dyn Error>>{
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
            .fetch_one(&self.sql_pool).await?;
        Ok(book)
    }


}