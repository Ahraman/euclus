-- Your SQL goes here

CREATE TABLE IF NOT EXISTS users (
        id              SERIAL PRIMARY KEY,
        username        VARCHAR NOT NULL UNIQUE,
        display_name    VARCHAR NOT NULL,
        pass            BYTEA NOT NULL,
        email           BYTEA NOT NULL
);
