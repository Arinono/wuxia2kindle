#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Epub {
    pub title: String,
    pub author: Option<String>,
    pub translator: Option<String>,
    pub chapters: Vec<(String, String)>,
    pub cover: Option<String>,
}
