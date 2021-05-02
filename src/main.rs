#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use primes::{Sieve, PrimeSet};
use rocket::response::content::{Json};
use std::time::{Instant};

fn bake_prime(min: Option<u64>, _max : Option<u64>) -> (f64, u64) {
    let start = Instant::now();
    let mut pset = Sieve::new();
    let min = match min {
        Some(x) => x,
        None => 0
    };
    let (_, prime) = pset.find(min);
    let duration = start.elapsed();
    (duration.as_secs() as f64 + duration.subsec_nanos() as f64 * 1e-9, prime)
}

fn to_json(duration: f64, prime: u64) -> Json<String> {
    Json(format!("{{ 'prime': {}, 'duration': {} }}", prime.to_string(), duration.to_string()))
}

#[get("/")]
fn root()  -> Json<String> {
    let (duration, prime) = bake_prime(None, None);
    to_json(duration, prime)
}

#[get("/range/<min>/<max>")]
fn range(min: u64, max: u64) -> Json<String> {
    let (duration, prime) = bake_prime(Some(min), Some(max));
    to_json(duration, prime)
}

fn main() {
    let rocket = rocket::ignite();
    let rocket = rocket.mount("/", routes![root, range]);
    rocket.launch();
}
