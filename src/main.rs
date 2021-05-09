#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate diesel;

use rocket::response::content::Json;

mod prime;
use prime::PrimeInterface;

#[get("/")]
fn root(prime_i: PrimeInterface) -> Json<String> {
    prime_i.random().to_json()
}

#[get("/first_greater_than/<min>")]
fn first_greater_than(prime_i: PrimeInterface, min: u64) -> Json<String> {
    prime_i.greater_than(min + 1, true).to_json()
}

#[get("/greater_than/<min>")]
fn greater_than(prime_i: PrimeInterface, min: u64) -> Json<String> {
    prime_i.greater_than(min + 1, false).to_json()
}

#[get("/at_position/<position>")]
fn at_position(prime_i: PrimeInterface, position: u64) -> Json<String> {
    prime_i.at_position(position).to_json()
}

fn main() {
    rocket::ignite()
        .attach(prime::PrimeInterface::fairing())
        .mount(
            "/",
            routes![root, first_greater_than, greater_than, at_position],
        )
        .launch();
}
