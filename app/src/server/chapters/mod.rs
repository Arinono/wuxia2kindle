use std::fmt::Display;

use serde::{Deserialize, Serialize};

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
