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
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

use self::models::{Activity, NewActivity};

pub fn create_activity<'a>(conn: &PgConnection, topic: &'a str, dy: &'a str, start_at: &'a str, end_at: &'a str) -> Activity {
    use schema::activities;

    let new_activity = NewActivity {
        topic: topic,
        dy: dy,
        start_at: start_at,
        end_at: end_at,
    };

    diesel::insert_into(activities::table)
        .values(&new_activity)
        .get_result(conn)
        .expect("Error saving new activity")
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}


