-- Your SQL goes here

CREATE TABLE IF NOT EXISTS pages (
    id              SERIAL PRIMARY KEY,
    title           VARCHAR(512) NOT NULL UNIQUE,
    rev_id          INTEGER NOT NULL DEFAULT 0,
    last_touched    TIMESTAMPTZ NOT NULL
);
