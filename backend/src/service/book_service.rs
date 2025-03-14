use crate::repository::book_repository::BookRepository;
use crate::model::book::{ Book, CreateBookRequest, UpdateBookRequest};
use uuid::Uuid;
use sqlx::Error;

#[derive(Clone)]
pub struct BookService{
    repository: BookRepository
}

impl BookService{
    pub fn new(repository: BookRepository) -> Self{
        BookService{repository}
    }

    pub async fn list_books(&self) -> Result<Vec<Book>, Error> {
        self.repository.list_books().await
    }

    pub async fn get_book_by_id(&self, id: Uuid) -> Result<Book, Error>  {
        self.repository.get_book_by_id(id).await
    }

    pub async fn delete_book_by_id(&self, id: Uuid) -> Result<bool, Error>  {
        self.repository.delete_book_by_id(id).await
    }

    pub async fn create_book(&self, request: CreateBookRequest) -> Result<Book, Error>  {
        let book = Book{
            id : Uuid::new_v4(),
            title : request.title,
            author : request.author,
            genre : request.genre,
            description : request.description,
            rating : request.rating,
            date_published : request.date_published
        };
        self.repository.save(book).await
    }

    pub async fn update_book_by_id(&self, id: Uuid, request: UpdateBookRequest) -> Result<Book, Error> {
        let existing_book = self.repository.get_book_by_id(id).await?;
 
        let book = Self::merge_books(existing_book, request);
        self.repository.save(book).await
    }

    fn merge_books(existing: Book, update: UpdateBookRequest) -> Book{
        Book { 
            id: existing.id, 
            title: update.title.unwrap_or_else(|| existing.title), 
            author: update.author.unwrap_or_else(|| existing.author), 
            genre: update.genre.unwrap_or_else(|| existing.genre), 
            description: update.description.or(existing.description), 
            rating: update.rating.or(existing.rating), 
            date_published: update.date_published.or(existing.date_published)
         }

    }
}
