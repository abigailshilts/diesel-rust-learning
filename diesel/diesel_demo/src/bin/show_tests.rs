extern crate diesel_demo;
extern crate diesel;

use self::diesel_demo::*;
use self::models::*;
use self::diesel::prelude::*;

fn main() {
    use diesel_demo::schema::tests::dsl::*;

    let connection = establish_connection();
    let results = tests.filter(published.eq(true))
        .limit(5)
        .load::<Test>(&connection)
        .expect("Error loading posts");

    println!("Displaying {} tests", results.len());
    for test in results {
        println!("{}", test.title);
        println!("----------\n");
        println!("{}", test.body);
    }
}