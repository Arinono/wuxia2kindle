pub use models::book::Book;
use serde::{Deserialize, Serialize};

pub mod get;
pub mod update;

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
