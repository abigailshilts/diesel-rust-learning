extern crate diesel_events;
extern crate diesel;

use self::diesel_events::*;
use std::io::{stdin, Read};

fn main() {
    let connection = establish_connection();

    println!("What would you like your activity to be?");
    let mut topic = String::new();
    stdin().read_line(&mut topic).unwrap();
    let topic = &topic[..(topic.len() - 1)]; // Drop the newline character
    println!("\nOk! Let's write {} (Press {} when finished)\n", topic, EOF);
    
    println!("\nWhat is the date for this?");
    let mut dy = String::new();
    stdin().read_to_string(&mut dy).unwrap();

    println!("\nWhat is the start time for this?");
    let mut start_at = String::new();
    stdin().read_to_string(&mut start_at).unwrap();

    println!("\nWhat is the end time for this?");
    let mut end_at = String::new();
    stdin().read_to_string(&mut end_at).unwrap();

    let activity = create_activity(&connection, topic, &dy, &start_at, &end_at);
    println!("\nSaved {} with id {}", topic, activity.id);
}

#[cfg(not(windows))]
const EOF: &'static str = "CTRL+D";

#[cfg(windows)]
const EOF: &'static str = "CTRL+Z";