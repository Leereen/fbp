use diesel::{Queryable};
use std::fmt;

use super::schema::primes;


#[derive(Queryable, Insertable)]
#[table_name = "primes"]
pub struct DBPrime {
    pub id: i64,
    pub value: i64,
}

impl PartialEq for DBPrime {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.value == other.value
    }
}

impl fmt::Debug for DBPrime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DBPrime")
            .field("id", &self.id)
            .field("value", &self.value)
            .finish()
    }
}
