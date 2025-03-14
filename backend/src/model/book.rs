use core::fmt;

use sqlx::FromRow;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};  // Utc for UTC times
use uuid::Uuid;  // Import the Uuid type
use rust_decimal::Decimal;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Book{
    pub id: Uuid,
    pub title: String,
    pub author: String,
    pub genre: String,
    pub description: Option<String>,
    pub rating: Option<Decimal>,
    pub date_published: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]

pub struct CreateBookRequest{
    pub title: String,
    pub author: String,
    pub genre: String,
    pub description: Option<String>,
    pub rating: Option<Decimal>,
    pub date_published: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]

pub struct UpdateBookRequest{
    pub title: Option<String>,
    pub author: Option<String>,
    pub genre: Option<String>,
    pub description: Option<String>,
    pub rating: Option<Decimal>,
    pub date_published: Option<DateTime<Utc>>,
}


impl fmt::Display for Book{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {}, {}, {:?}, {:?}, {:?})", self.id, self.title, self.author, self.genre, self.description, self.rating, self.date_published)
    }
}


impl fmt::Display for CreateBookRequest{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {}, {:?}, {:?}, {:?})", self.title, self.author, self.genre, self.description, self.rating, self.date_published)
    }
}

impl fmt::Display for UpdateBookRequest{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:?}, {:?}, {:?}, {:?}, {:?}, {:?})", self.title, self.author, self.genre, self.description, self.rating, self.date_published)
    }
}