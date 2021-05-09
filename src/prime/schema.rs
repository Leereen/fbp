use diesel;

diesel::table! {
    primes (id) {
        id -> BigInt,
        value -> BigInt,
    }
}
