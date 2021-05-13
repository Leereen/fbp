// use diesel;
// use diesel::prelude::QueryResult;
// use diesel::insert_into;
use crate::diesel::ExpressionMethods;
use crate::diesel::QueryDsl;
use crate::diesel::RunQueryDsl;

use super::utils::Prime;
use super::model::DBPrime;
use super::schema::primes;
use super::schema::primes::dsl::*;


pub fn get_position(conn: &diesel::SqliteConnection, position: u64) -> Option<Prime> {
    match primes.find(position as i64).first::<DBPrime>(conn) {
        Ok(result) => return Some(Prime::from(result)),
        Err(_) => return None,
    }
}
