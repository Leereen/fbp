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

#[get("/greater_than/<min>")]
fn from(min: u64) -> Json<String> {
    to_json(prime::greater_than(min))
}

#[get("/at_position/<idx>")]
fn position(idx: u64) -> Json<String> {
    to_json(prime::at_position(idx))
}

fn main() {
    let rocket = rocket::ignite();
    let rocket = rocket.mount("/", routes![root, from, position]);
    rocket.launch();
}
