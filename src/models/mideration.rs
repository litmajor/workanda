#[derive(Debug, Deserialize)]
pub struct ModerationRequest {
    pub message_id: i32,
    pub action: String, // e.g., "delete" or "modify"
    pub new_content: Option<String>, // Optional new content if modifying
}