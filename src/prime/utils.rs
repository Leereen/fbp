use super::model;
use rocket::response::content::Json;
use std::fmt;

#[derive(Copy, Clone)]
pub struct Prime {
    pub value: u64,
    pub position: u64,
    pub duration: f64,
    pub new: bool,
}

impl From<model::DBPrime> for Prime {
    fn from(item: model::DBPrime) -> Prime {
        Prime {
            value: item.value as u64,
            position: item.id as u64,
            duration: 0.0,
            new: false,
        }
    }
}

impl From<Prime> for model::DBPrime {
    fn from(item: Prime) -> model::DBPrime {
        model::DBPrime {
            id: item.position as i64,
            value: item.value as i64,
        }
    }
}

impl fmt::Debug for Prime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Prime")
            .field("value", &self.value)
            .field("position", &self.position)
            .field("duration", &self.duration)
            .field("new", &self.new)
            .finish()
    }
}

impl PartialEq for Prime {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
            && self.position == other.position
            && self.duration == other.duration
            && self.new == other.new
    }
}

impl Prime {
    pub fn new(value: u64, position: u64, duration: f64, new: bool) -> Prime {
        Prime {
            value: value,
            position: position,
            duration: duration,
            new: new,
        }
    }

    pub fn to_json(&self) -> Json<String> {
        Json(format!(
            "{{ \"prime\": {}, \"position\": {}, \"duration\": {}, \"new\": {} }}",
            self.value.to_string(),
            self.position.to_string(),
            self.duration.to_string(),
            self.new.to_string()
        ))
    }
}

#[cfg(test)]
mod tests {

    use super::model;
    use super::Prime;

    #[test]
    fn prime_to_dbprime() {
        assert_eq!(
            model::DBPrime {
                id: 123456789,
                value: 987654321
            },
            model::DBPrime::from(Prime::new(987654321, 123456789, 0.0, false))
        );
        assert_eq!(
            model::DBPrime {
                id: -123456789,
                value: -987654321
            },
            model::DBPrime::from(Prime::new(
                18446744072721897295,
                18446744073586094827,
                0.0,
                false
            ))
        );
    }

    #[test]
    fn dbprime_to_prime() {
        assert_eq!(
            Prime::new(987654321, 123456789, 0.0, false),
            Prime::from(model::DBPrime {
                id: 123456789,
                value: 987654321,
            })
        );
        assert_eq!(
            Prime::new(18446744072721897295, 18446744073586094827, 0.0, false),
            Prime::from(model::DBPrime {
                id: -123456789,
                value: -987654321,
            })
        );
    }

    #[test]
    fn prime_to_prime() {
        let primes = [
            Prime::new(0, 0, 0.0, false),
            Prime::new(12345678900987654321, 9876543210123456789, 0.0, false),
            Prime::new(u64::MAX, u64::MAX, 0.0, false),
        ];
        for prime in primes.iter() {
            assert_eq!(*prime, Prime::from(model::DBPrime::from(*prime)));
        }
        let prime1 = Prime::new(0, 0, 0.0, false);
        let prime2 = Prime::new(0, 0, f64::MAX, true);
        assert_eq!(prime1, Prime::from(model::DBPrime::from(prime2)));
    }
}
