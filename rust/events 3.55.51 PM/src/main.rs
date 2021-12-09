#![feature(decl_macro)]
#[macro_use]
extern crate diesel;

use diesel::prelude::*;
use diesel::pg::PgConnection;
//use dotenv::dotenv;
use rocket_contrib::databases::database;
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};
use models::{Activity, NewActivity};

mod models;
mod schema;
mod error;

use error::CustomError;
use rocket::response::status::Created;

use rocket::*;
//use rocket::request::Form;

//use rocket::response::content::Json;

#[database("activities_db")]
pub struct DBPool(PgConnection);

#[derive(Deserialize)]
struct ActivityJSON {
    pub topic: String,
    pub dy: String,
    pub start_at: String,
    pub end_at: String,
}

#[derive(Serialize)]
struct NewActivityResponse{
    activity: Activity,
}

#[get("/hello")]
fn hello() -> Json<&'static str> {
  Json("{
    'status': 'success',
    'message': 'Hello API!'
  }")
}

#[post("/activity", format = "json", data = "<activity>")]
async fn post_activity(
    conn: DBPool,
    activity: Json<ActivityJSON>,
) -> Result<Created<Json<NewActivityResponse>>, CustomError>{
    let new_activity: Activity = conn
        .run(move |c| {
            diesel::insert_into(schema::activities::table)
            .values(NewActivity {
                topic: &activity.topic,
                dy: &activity.dy,
                start_at: &activity.start_at,
                end_at: &activity.end_at,
            })
            .get_result(c)
        })
        .await?;
    let response = NewActivityResponse {
        activity: new_activity,
    };
    Ok(Created::new("/activity").body(Json(activity)))

}

fn main() {
    rocket::ignite()
        //.mount("/", routes![root, static_files])
        .mount("/api", routes![hello, post_activity])
        .attach(DBPool::fairing())
        .launch();
}
