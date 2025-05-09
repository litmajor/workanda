#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Agency {
    pub id: Uuid,
    pub name: String,
    pub owner_id: Uuid,
    pub team_ids: Vec<Uuid>,
    pub verified: bool,
    pub reputation_score: f64,
    pub categories: Vec<String>,
    pub projects_completed: i32,
    pub avg_delivery_time: i32,
}