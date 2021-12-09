#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

pub mod schema;
pub mod models;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set")
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

use self::models::{Test, NewTest};

pub fn create_test<'a>(conn: &PgConnection, title: &'a str, body: &'a str) -> Test {
    use schema::tests;

    let new_test = NewTest {
        title: title,
        body: body,
    };

    diesel::insert_into(tests::table)
        .values(&new_test)
        .get_result(conn)
        .expect("Error saving new test")
}