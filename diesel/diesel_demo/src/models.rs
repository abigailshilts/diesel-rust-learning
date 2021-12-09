#[derive(Queryable)]
pub struct Test {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

use super::schema::tests;

#[derive(Insertable)]
#[table_name="tests"]
pub struct NewTest<'a> {
    pub title: &'a str,
    pub body: &'a str,
}