#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
extern crate rusqlite;

mod db;
mod routes;

// GLOBALS
const DATABASE: &str = "users.db";
const USER_ID_LENGTH: usize = 6;

fn main() {
    // CREATE DATABASE
    match crate::db::create::create() {
        Ok(result) => println!("Connected to database: {:?}", result),
        Err(e) => panic!("Could not connect to database: {:?}", e),
    };

    // Setup rocket
    rocket().launch();
}

// Inits rocket
pub fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/register", routes![routes::register])
        .mount("/pair", routes![routes::pair])
        .mount("/send", routes![routes::send])
        .mount("/key", routes![routes::get_key])
        .register(catchers![routes::not_found])
}
