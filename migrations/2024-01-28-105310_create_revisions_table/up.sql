-- Your SQL goes here

CREATE TABLE IF NOT EXISTS revisions (
    id              SERIAL PRIMARY KEY,
    page_id         INTEGER NOT NULL
                        REFERENCES pages(id)
                            ON DELETE RESTRICT,
    user_id         INTEGER
                        REFERENCES users(id)
                            ON DELETE SET NULL,
    parent_id       INTEGER DEFAULT NULL
                        REFERENCES revisions(id)
                            ON DELETE CASCADE,
    creation_time   TIMESTAMPTZ
);
