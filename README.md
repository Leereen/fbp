[![Travis Build Status](https://travis-ci.com/Leereen/fbp.svg)](https://travis-ci.com/Leereen/fbp)
# Freshly Baked Primes

Freshly Baked Primes is a basic pet projet I started to discover the
[Rust](https://doc.rust-lang.org/book/) programming langage.

It is a very simple HTTP API returning prime numbers.

For now, it uses:

* [Rocket](https://rocket.rs/) for the HTTP API,
* [Primes](https://docs.rs/primes/0.3.0/primes/) to generate the prime numbers.


## How-to


### Compile

As a pure Rust repo for now, the compilation is very simple (given you have
Rust already; if not, [see here](https://www.rust-lang.org/tools/install)):

```
git clone git@github.com:Leereen/fbp.git
cd fbp
cargo build
```


### Run

Again, very simple:

```
cargo run
```


### HTTP API usage

The API is composed of 4 routes accessible with the GET HTTP method. They all
return a JSON with the following structure:

```
{
    "value":    <u64>,   // the prime number
    "position": <usize>, // the prime number position, given '1' has position '0'
                         // (then '2' is first, '3' second, '5' third, ...)
    "duration": <f64>,   // the time FBP took to generate the prime number
    "new":      <bool>   // if the prime has never been calculated by FBP
                         // (always true, except for '1' which is hard-coded -
                         //  will be used in the 'database' feature)
}
```

* `/` returns a prime randomly picked from the database.
* `/first_grater_than/<N: int>` returns the first prime number strictly greater than `N`.
* `/greater_than/<N: int>` returns a prime from the database greater than N.
  If the database does not contain such prime, falls back to previous route.
* `/at_position/<k: int>` returns the prime number at position `k`.

The database grows with time, as every prime calculated are stored.


## License

You can read the `LICENSE` file or trust me when I say its MIT.