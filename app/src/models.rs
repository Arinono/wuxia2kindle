use serde::{Deserialize, Serialize};
use sqlx::types::{JsonValue, chrono::{DateTime, Utc}};
use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct Book {
    pub id: i32,
    pub name: String,
    pub chapter_count: Option<i16>,
    pub author: Option<String>,
    pub translator: Option<String>,
    pub cover: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
pub struct Chapter {
    pub id: i32,
    pub book_id: i32,
    pub name: String,
    pub content: String,
    pub number_in_book: i16,
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

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum ExportKinds {
    Anthology,
    FullBook(i32),
    SingleChapter(i32),
    // will error if there is a blank spot in the range
    ChaptersRange { book_id: i32, chapters: (i16, i16) },
}

impl From<JsonValue> for ExportKinds {
    fn from(value: JsonValue) -> Self {
        serde_json::from_value(value).unwrap()
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct Export {
    pub id: i32,
    pub meta: ExportKinds,
    pub created_at: DateTime<Utc>,
    pub processing_started_at: Option<DateTime<Utc>>,
    pub processed_at: Option<DateTime<Utc>>,
    pub sent: bool,
    pub error: Option<String>,
}

enum ExportState {
    Created,
    Processing,
    Processed,
    Sent,
    Failed,
}

impl Display for ExportState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let state = match self {
            ExportState::Sent => "sent",
            ExportState::Processed => "processed",
            ExportState::Processing => "processing",
            ExportState::Created => "created",
            ExportState::Failed => "failed",
        };

        write!(f, "{state}")
    }
}

impl Export {
    fn get_state(&self) -> ExportState {
        match self.error {
            Some(_) => ExportState::Failed,
            None => match self.sent {
                true => ExportState::Sent,
                false => match self.processed_at {
                    Some(_) => ExportState::Processed,
                    None => match self.processing_started_at {
                        Some(_) => ExportState::Processing,
                        None => ExportState::Created,
                    },
                },
            },
        }
    }
}

impl Display for Export {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} -> {}", self.get_state(), self.meta)
    }
}
