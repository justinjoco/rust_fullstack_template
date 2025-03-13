use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};  // Utc for UTC times
use uuid::Uuid;  // Import the Uuid type
#[derive(Debug, Serialize, Deserialize)]
pub struct Book{
    pub id: Uuid,
    pub title: String,
    pub author: String,
    pub genre: String,
    pub description: Option<String>,
    pub rating: Option<f64>,
    pub date_published: Option<DateTime<Utc>>,
}

impl Book {
    pub fn new(id: Uuid, title: String, author: String, genre: String, description: Option<String>, rating: Option<f64>, date_published: Option<DateTime<Utc>>) -> Self {
        Book {
            id,  // Automatically generate a new UUID
            title,
            author,
            genre,
            description,
            rating,
            date_published
        }
    }
}
