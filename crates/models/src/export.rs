use chrono::{DateTime, Utc};
use std::fmt::Display;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "serde")]
use serde_json::Value as JsonValue;

#[cfg(feature = "serde")]
mod date_fmt {
    use chrono::{DateTime, NaiveDateTime, Utc};
    use serde::{self, Deserialize, Deserializer, Serializer};

    const FORMAT: &'static str = "%Y-%m-%d %H:%M:%S";

    pub fn serialize<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let dt = NaiveDateTime::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)?;
        Ok(DateTime::<Utc>::from_naive_utc_and_offset(dt, Utc))
    }
}

#[cfg(feature = "serde")]
mod opt_date_fmt {
    use chrono::{DateTime, NaiveDateTime, Utc};
    use serde::{self, Deserialize, Deserializer, Serializer};

    const FORMAT: &'static str = "%Y-%m-%d %H:%M:%S";

    pub fn serialize<S>(date: &Option<DateTime<Utc>>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = match date {
            Some(date) => format!("{}", date.format(FORMAT)),
            None => "".to_string(),
        };
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<DateTime<Utc>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let dt = match s.is_empty() {
            true => return Ok(None),
            false => {
                let dt =
                    NaiveDateTime::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)?;
                Some(DateTime::<Utc>::from_naive_utc_and_offset(dt, Utc))
            }
        };

        Ok(dt)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Export {
    pub id: i32,
    pub meta: ExportKinds,
    #[cfg_attr(feature = "serde", serde(with = "date_fmt"))]
    pub created_at: DateTime<Utc>,
    #[cfg_attr(feature = "serde", serde(with = "opt_date_fmt"))]
    pub processing_started_at: Option<DateTime<Utc>>,
    #[cfg_attr(feature = "serde", serde(with = "opt_date_fmt"))]
    pub processed_at: Option<DateTime<Utc>>,
    pub sent: bool,
    pub error: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ExportKinds {
    Anthology,
    FullBook(i32),
    SingleChapter(i32),
    // will error if there is a blank spot in the range
    ChaptersRange { book_id: i32, chapters: (i32, i32) },
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum ExportState {
    Created,
    Processing,
    Processed,
    Sent,
    Failed,
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

#[cfg(feature = "serde")]
impl From<JsonValue> for ExportKinds {
    fn from(value: JsonValue) -> Self {
        serde_json::from_value(value).unwrap()
    }
}
