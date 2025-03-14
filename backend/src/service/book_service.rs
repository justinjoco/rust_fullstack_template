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
use crate::model::book::{ Book, CreateBookRequest, UpdateBookRequest};
use uuid::Uuid;

#[derive(Clone)]
pub struct BookService{
    repository: BookRepository
}

impl BookService{
    pub fn new(repository: BookRepository) -> Self{
        BookService{repository}
    }

    pub async fn list_books(&self) -> Vec<Book> {
        self.repository.list_books().await.unwrap()
    }

    pub async fn get_book_by_id(&self, id: Uuid) -> Book {
        self.repository.get_book_by_id(id).await.unwrap()
    }

    pub async fn delete_book_by_id(&self, id: Uuid) -> bool {
        self.repository.delete_book_by_id(id).await.unwrap()
    }

    pub async fn create_book(&self, request: CreateBookRequest) -> Book {
        let book = Book{
            id : Uuid::new_v4(),
            title : request.title,
            author : request.author,
            genre : request.genre,
            description : request.description,
            rating : request.rating,
            date_published : request.date_published
        };
        self.repository.save(book).await.unwrap()
    }

    pub async fn update_book_by_id(&self, id: Uuid, request: UpdateBookRequest) -> Book {
        let existing_book =         self.repository.get_book_by_id(id).await.unwrap();
        let book = Self::merge_books(existing_book, request);
        self.repository.save(book).await.unwrap()
    }

    fn merge_books(existing: Book, update: UpdateBookRequest) -> Book{
        Book { 
            id: existing.id, 
            title: update.title.unwrap_or(existing.title), 
            author: update.author.unwrap_or(existing.author), 
            genre: update.genre.unwrap_or(existing.genre), 
            description: update.description.or(existing.description), 
            rating: update.rating.or(existing.rating), 
            date_published: update.date_published.or(existing.date_published)
         }

    }
}
