extern crate diesel_events;
extern crate diesel;

use self::diesel::prelude::*;
use self::diesel_events::*;
use std::env::args;
use std::io::{stdin, Read};

fn main() {
    use diesel_events::schema::activities::dsl::*;

    println!("enter topic to delete");
    let mut pattern = String::new();
    stdin().read_to_string(&mut pattern).unwrap();
    let pattern = &pattern[..(pattern.len() - 1)];

    let connection = establish_connection();
    let num_deleted = diesel::delete(activities.filter(topic.like(pattern)))
        .execute(&connection)
        .expect("Error deleting topics");

    println!("Deleted {} topics", num_deleted);
}