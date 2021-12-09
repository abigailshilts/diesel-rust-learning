

#[derive(Queryable)]
pub struct Activity {
    pub id: i32,
    pub topic: String,
    pub dy: String,
    pub start_at: String,
    pub end_at: String,
}

use super::schema::activities;

#[derive(Insertable)]
#[table_name="activities"]
pub struct NewActivity<'a> {
    pub topic: &'a str,
    pub dy: &'a str,
    pub start_at: &'a str,
    pub end_at: &'a str,
}