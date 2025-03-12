#[derive(Debug, Deserialize)]
pub struct SearchQuery {
    pub query: String,
    pub user_id: i32,
}