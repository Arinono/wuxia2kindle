pub mod add;
pub mod get;

use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum Responses {
    AddChapter { success: bool },
    GetChapters { data: Vec<Chapter> },
    GetChapter { data: Chapter },
    Empty,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AddChapter {
    book: String,
    name: String,
    content: String,
    number_in_book: i32,
    author: Option<String>,
    translator: Option<String>,
}

impl Display for AddChapter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} - {} #{}", self.book, self.name, self.number_in_book)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Chapter {
    pub id: i32,
    pub book_id: i32,
    pub name: String,
    pub content: String,
    pub number_in_book: i32,
}

impl Display for Chapter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({}) {} #{}",
            self.book_id, self.name, self.number_in_book
        )
    }
}
