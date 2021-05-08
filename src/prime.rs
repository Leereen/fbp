use primes::{PrimeSet, Sieve};
use std::time::Instant;

pub struct Prime {
    pub value: u64,
    pub new: bool,
    pub duration: f64,
}

pub fn greater_than(min: u64) -> Prime {
    duration_wrapper(_greater_than, min)
}

pub fn at_position(idx: u64) -> Prime {
    match idx {
        0 => Prime {
            value: 0,
            duration: 0.0,
            new: false,
        },
        1 => Prime {
            value: 1,
            duration: 0.0,
            new: false,
        },
        _ => duration_wrapper(_at_position, idx - 1),
    }
}

pub fn random() -> Prime {
    Prime {
        value: 0,
        duration: 0.0,
        new: false,
    }
}

fn duration_wrapper(function: fn(u64) -> Prime, arg: u64) -> Prime {
    let start = Instant::now();
    let mut result = function(arg);
    let duration = start.elapsed();
    result.duration = duration.as_secs() as f64 + duration.subsec_nanos() as f64 * 1e-9;
    result
}

fn _greater_than(min: u64) -> Prime {
    let mut pset = Sieve::new();
    Prime {
        value: pset.find(min).1,
        duration: 0.0,
        new: false,
    }
}

fn _at_position(idx: u64) -> Prime {
    let mut pset = Sieve::new();
    for (_idx, prime) in pset.iter().enumerate().skip(idx as usize - 1).take(1) {
        return Prime {
            value: prime,
            duration: 0.0,
            new: false,
        };
    }
    Prime {
        value: 0,
        duration: 0.0,
        new: false,
    }
}
