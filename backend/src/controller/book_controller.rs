pub trait Controller {
    pub fn list_all_books(&self);
    pub fn get_book_by_id(&self);
    pub fn delete_book_by_id(&self);
    pub fn update_book_by_id(&self);
    pub fn create_book(&self);
}
pub struct BookController{
    
}

impl BookController{
    pub fn new() -> Self{

    }
}

impl Controller for BookController{
    
}