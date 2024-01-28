-- Your SQL goes here

CREATE TABLE IF NOT EXISTS contents (
    id      SERIAL PRIMARY KEY,
    text_id INTEGER NOT NULL
                REFERENCES texts(id)
                    ON DELETE RESTRICT
);
