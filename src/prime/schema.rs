diesel::table! {
    primes (id) {
        id -> BigInt,    // prime position (as signed)
        value -> BigInt, // prime value
    }
}
