-- Your SQL goes here

CREATE TABLE IF NOT EXISTS users (
        id              SERIAL PRIMARY KEY,
        username        VARCHAR(255) NOT NULL UNIQUE,
        display_name    VARCHAR(255) NOT NULL,
        pass            BYTEA NOT NULL,
        email           BYTEA NOT NULL
);
