-- Your SQL goes here

CREATE TABLE IF NOT EXISTS slot_roles (
    id      SERIAL PRIMARY KEY,
    title   VARCHAR(64) UNIQUE NOT NULL
);
