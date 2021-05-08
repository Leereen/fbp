#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket::response::content::Json;

mod prime;

fn to_json(prime: prime::Prime) -> Json<String> {
    Json(format!(
        "{{ 'prime': {}, 'duration': {}, 'new': {} }}",
        prime.value.to_string(),
        prime.duration.to_string(),
        prime.new.to_string()
    ))
}

#[get("/")]
fn root() -> Json<String> {
    to_json(prime::random())
}

#[get("/first_greather_than/<min>")]
fn first_greater_than(min: u64) -> Json<String> {
    to_json(prime::greater_than(min, true))
}

#[get("/greater_than/<min>")]
fn greater_than(min: u64) -> Json<String> {
    to_json(prime::greater_than(min, false))
}

#[get("/at_position/<idx>")]
fn at_position(idx: u64) -> Json<String> {
    to_json(prime::at_position(idx))
}

fn main() {
    let rocket = rocket::ignite();
    let rocket = rocket.mount(
        "/",
        routes![root, first_greater_than, greater_than, at_position],
    );
    rocket.launch();
}
