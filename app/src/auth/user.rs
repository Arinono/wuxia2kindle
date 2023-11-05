#[derive(Debug)]
pub struct User {
    pub id: i32,
    pub discord_id: Option<String>,
    pub username: String,
    pub avatar: Option<String>,
}
