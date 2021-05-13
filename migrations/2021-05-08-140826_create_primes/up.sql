-- Your SQL goes here
CREATE TABLE "primes" (
    id     BIGINT PRIMARY KEY UNIQUE NOT NULL,
    value  BIGINT UNIQUE NOT NULL
);

INSERT INTO "primes"(id, value) VALUES (0, 1);
INSERT INTO "primes"(id, value) VALUES (1, 2);
INSERT INTO "primes"(id, value) VALUES (2, 3);
INSERT INTO "primes"(id, value) VALUES (3, 5);
INSERT INTO "primes"(id, value) VALUES (4, 7);
INSERT INTO "primes"(id, value) VALUES (5, 11);
INSERT INTO "primes"(id, value) VALUES (6, 13);
