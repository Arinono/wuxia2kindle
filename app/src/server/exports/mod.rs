pub mod add;

use serde::{Deserialize, Serialize};
use sqlx::types::{
    chrono::{DateTime, Utc},
    JsonValue,
};
use std::fmt::Display;

#[derive(Debug, Deserialize, Serialize)]
pub enum Responses {
    Empty,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AddToQueue {
    book_id: i32,
    from: i32,
    to: i32,
}

impl Display for ExportKinds {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExportKinds::ChaptersRange { book_id, chapters } => write!(
                f,
                "{}: Chapters from {} to {}",
                book_id, chapters.0, chapters.1
            ),
            _ => todo!(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum ExportKinds {
    Anthology,
    FullBook(i32),
    SingleChapter(i32),
    // will error if there is a blank spot in the range
    ChaptersRange { book_id: i32, chapters: (i32, i32) },
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
