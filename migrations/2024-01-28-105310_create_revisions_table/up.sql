-- Your SQL goes here

CREATE TABLE IF NOT EXISTS revisions (
    id              SERIAL PRIMARY KEY,
    page_id         INTEGER
                        REFERENCES pages(id),
    user_id         INTEGER
                        REFERENCES users(id),
    prev_rev        INTEGER DEFAULT NULL
                        REFERENCES revisions(id),
    creation_time   TIMESTAMPTZ
);
