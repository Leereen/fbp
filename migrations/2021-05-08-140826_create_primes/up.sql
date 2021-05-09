-- Your SQL goes here
CREATE TABLE "primes" (
    id     INTEGER PRIMARY KEY UNIQUE NOT NULL,
    value  INTEGER UNIQUE NOT NULL
);

INSERT INTO
    "primes"(id, value)
VALUES
    (0, 1);
