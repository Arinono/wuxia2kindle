pub mod add;
mod epub;

use serde::{Deserialize, Serialize};

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
