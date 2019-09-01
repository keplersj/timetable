#[macro_use]
extern crate diesel;
extern crate dotenv;
mod models;
mod schema;

use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

use diesel::prelude::*;
use schema::*;
use models::*;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

fn main() {
    use schema::scheduled_requests::dsl::*;

    let connection = establish_connection();
    let results = scheduled_requests.filter(executed.eq(false))
        .limit(5)
        .load::<ScheduledRequest>(&connection)
        .expect("Error loading requests");

    println!("Displaying {} requests", results.len());
    for post in results {
        println!("{}", post.id);
        println!("----------\n");
        println!("{}", post.hook);
        println!("----------\n");
        println!("{:?}", post.time);
    }
}
