// use diesel;
// use diesel::prelude::*;

// use super::utils;
// use super::model;
// use super::schema;
// use super::schema::primes::dsl::*;

// fn write_schema(conn: &diesel::SqliteConnection) {
//     let results = primes
//         .limit(3)
//         .load::<model::DBPrime>(conn)
//         .expect("Error loading primes");
// }

// fn add(conn: &diesel::SqliteConnection, prime: Prime) -> bool {
//     conn.execute(
//         "INSERT INTO prime (id, position) VALUES ('?1', '?2')",
//         params![prime.value, prime.position],
//     ) {
//         Ok(updated) => {
//             match update {
//                 0 => return false,
//                 1 => return true,
//             }
//         }
//     };
// }

// fn get_position(conn: &diesel::SqliteConnection, position: usize) -> Prime {
//     let mut stmt = conn.prepare(
//         "SELECT id FROM prime WHERE position = '?1'"
//     )?;
//     let prime_iter = stmt.query_map(params![position], |row| {
//         Ok(Prime::new(row.get(0), position, 0.0, true))
//     })?;
// }
