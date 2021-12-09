extern crate diesel_events;
extern crate diesel;

use self::diesel::prelude::*;
use self::diesel_events::*;
use self::models::Activity;
use std::env::args;
use std::io::{stdin, Read};

fn main() {
    use diesel_events::schema::activities::dsl::{activities, start_at};

    let id = args().nth(1).expect("change_star_activities requires an activity id")
        .parse::<i32>().expect("Invalid ID");
    let connection = establish_connection();

    println!("enter new start time");
    let mut new_start = String::new();
    stdin().read_to_string(&mut new_start).unwrap();

    let activity = diesel::update(activities.find(id))
        .set(start_at.eq(new_start))
        .get_result::<Activity>(&connection)
        .expect(&format!("Unable to find activity {}", id));
    println!("New start time for {}", activity.topic);
}