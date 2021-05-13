use primes::{PrimeSet, Sieve};
use std::time::Instant;
// use rocket_contrib::databases::diesel;
// use rocket_contrib::databases::diesel::prelude::*;

mod db;
mod model;
mod schema;
mod utils;

use utils::Prime;

#[database("mydb")]
pub struct PrimeInterface(diesel::SqliteConnection);

impl PrimeInterface {
    pub fn greater_than(&self, min: u64, very_next: bool) -> Prime {
        if very_next {
            self.duration_wrapper(PrimeInterface::_first_greater_than, min)
        } else {
            self.duration_wrapper(PrimeInterface::_greater_than, min)
        }
    }

    pub fn at_position(&self, position: u64) -> Prime {
        self.duration_wrapper(PrimeInterface::_at_position, position)
    }

    pub fn random(&self) -> Prime {
        self.duration_wrapper(PrimeInterface::_get_random, 0)
    }

    fn duration_wrapper(&self, function: fn(&PrimeInterface, u64) -> Prime, arg: u64) -> Prime {
        let start = Instant::now();
        let mut result = function(self, arg);
        let duration = start.elapsed();
        result.duration = duration.as_secs() as f64 + duration.subsec_nanos() as f64 * 1e-9;
        result
    }

    fn _get_random(&self, _: u64) -> Prime {
        db::get_random(self)
    }

    fn _first_greater_than(&self, min: u64) -> Prime {
        let mut pset = Sieve::new();
        // primes starts at '2' (which is the 1st prime) with idx of '0',
        // hence the "idx + 1"
        let (idx, value) = pset.find(min);
        let prime = Prime::new(value, (idx + 1) as u64, 0.0, true);
        db::insert(self, prime);
        prime
    }

    fn _greater_than(&self, min: u64) -> Prime {
        match db::get_greater_than(self, min) {
            Some(result) => result,
            None => {
                let result = self._first_greater_than(min);
                db::insert(self, result);
                result
            }
        }
    }

    fn _at_position(&self, position: u64) -> Prime {
        match db::get_position(self, position) {
            Some(result) => result,
            None => {
                let mut result = Prime::new(1, 0, 0.0, false);
                let mut pset = Sieve::new();
                for (idx, prime) in pset.iter().enumerate().skip(position as usize - 1).take(1) {
                    // primes starts at '2' (which is the 1st prime) with idx of '0',
                    // hence the "idx + 1"
                    result = Prime::new(prime, (idx + 1) as u64, 0.0, true);
                }
                db::insert(self, result);
                result
            }
        }
    }
}
