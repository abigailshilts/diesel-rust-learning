#![feature(decl_macro)]
#[macro_use] 
extern crate rocket;
extern crate dotenv;
extern crate events_restart;
extern crate diesel;

use serde::Serialize;
use rocket_contrib::templates::Template;
use self::events_restart::*;
use self::models::*;
use self::diesel::prelude::*;
use std::env;
use std::io::{stdin, Read};
use rocket::response::content::Json;

#[get("/hello")]
fn hello() -> Json<&'static str> {
  Json("{
    'status': 'success',
    'message': 'Hello API!'
  }")
}

//#[get("/output")]
/*fn output() -> Json<&'static str> {
    use events_restart::schema::activities::dsl::*;

    let connection = establish_connection();
    let results = activities.filter(id.eq(1))
        .load::<Activity>(&connection)
        .expect("Error loading Activities");
    
    //return results[0].topic;

    
    //Json(format!("{}", results[0].topic))
}*/



#[get("/")]
fn index() -> Template {

  use events_restart::schema::activities::dsl::*;

  let connection = establish_connection();
  let results = activities.filter(id.eq(1))
    .load::<Activity>(&connection)
    .expect("Error loading Activities");

  Template::render("home", &results[0])
}

fn main() {
    rocket::ignite()
        .mount("/api", routes![hello])
        .mount("/", routes![index])
        .attach(Template::fairing())
        .launch();
}
