#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Book {
    pub id: i32,
    pub name: String,
    pub chapter_count: Option<i32>,
    pub author: Option<String>,
    pub translator: Option<String>,
    pub cover: Option<String>,
}
