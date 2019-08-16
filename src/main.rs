#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
extern crate rusqlite;
extern crate tokio;

mod db;
mod push;
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
        .mount("/register", routes![routes::register::register])
        .mount("/pair", routes![routes::pair::pair])
        .mount("/send", routes![routes::send::send])
        .mount("/key", routes![routes::key::get_key])
        .register(catchers![routes::generics::not_found])
}
