use crate::diesel::ExpressionMethods;
use crate::diesel::QueryDsl;
use crate::diesel::RunQueryDsl;

use super::model::DBPrime;
use super::schema::primes::dsl::*;
use super::utils::Prime;

pub fn insert(conn: &diesel::SqliteConnection, prime: Prime) -> bool {
    let db_prime = DBPrime::from(prime);
    match diesel::insert_into(primes).values(&db_prime).execute(conn) {
        Ok(_) => true,
        Err(_) => false,
    }
}

pub fn get_greater_than(conn: &diesel::SqliteConnection, lower_bound: u64) -> Option<Prime> {
    match primes
        .filter(value.gt(lower_bound as i64))
        .first::<DBPrime>(conn)
    {
        Ok(result) => return Some(Prime::from(result)),
        Err(_) => return None,
    }
}

pub fn get_position(conn: &diesel::SqliteConnection, position: u64) -> Option<Prime> {
    match primes.find(position as i64).first::<DBPrime>(conn) {
        Ok(result) => return Some(Prime::from(result)),
        Err(_) => return None,
    }
}

no_arg_sql_function!(
    RANDOM,
    diesel::sql_types::Integer,
    "Represents the SQL RANDOM() function"
);

pub fn get_random(conn: &diesel::SqliteConnection) -> Prime {
    Prime::from(
        primes
            .order(RANDOM)
            .first::<DBPrime>(conn)
            .expect("That's weird"),
    )
}
