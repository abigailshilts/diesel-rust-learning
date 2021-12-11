use diesel::Queryable;
use serde::Serialize;

#[derive(Queryable, Serialize)]
pub struct Activity {
    pub id: i32,
    pub topic: String,
    pub dy: String,
    pub start_at: String,
    pub end_at: String,
}
