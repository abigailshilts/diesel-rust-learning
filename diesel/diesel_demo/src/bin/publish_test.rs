extern crate diesel_demo;
extern crate diesel;

use self::diesel::prelude::*;
use self::diesel_demo::*;
use self::models::Test;
use std::env::args;

fn main() {
    use diesel_demo::schema::tests::dsl::{tests, published};

    let id = args().nth(1).expect("publish_test requires a test id")
        .parse::<i32>().expect("Invalid ID");
    let connection = establish_connection();

    let test = diesel::update(tests.find(id))
        .set(published.eq(true))
        .get_result::<Test>(&connection)
        .expect(&format!("Unable to find test {}", id));
    println!("Published test {}", test.title);
}