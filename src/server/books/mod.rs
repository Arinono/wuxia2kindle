use serde::{Deserialize, Serialize};

pub mod get;
pub mod update;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Book {
    pub id: i32,
    pub name: String,
    pub chapter_count: Option<i32>,
    pub author: Option<String>,
    pub translator: Option<String>,
    pub cover: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateBook {
    pub name: Option<String>,
    pub cover: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Responses {
    GetBook { data: Book },
    GetBooks { data: Vec<Book> },
    Empty,
}
