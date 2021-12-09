extern crate diesel_events;
extern crate diesel;

use self::diesel_events::*;
use self::models::*;
use self::diesel::prelude::*;
use std::env;
use std::io::{stdin, Read};

fn main(){
    use diesel_events::schema::activities::dsl::*;

    println!("enter date to search by");
    let mut args = String::new();
    stdin().read_to_string(&mut args).unwrap();

    let connection = establish_connection();
    let results = activities.filter(dy.eq(&args))
        .load::<Activity>(&connection)
        .expect("Error loading Activities");

    println!("Displaying {} Activities", results.len());
    for activity in results {
        println!("{}", activity.topic);
        println!("----------\n");
        println!("{:?}", activity.start_at);
        println!("----------\n");
        println!("{:?}", activity.end_at);
    }

}