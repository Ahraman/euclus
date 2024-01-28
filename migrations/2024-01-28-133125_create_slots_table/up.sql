-- Your SQL goes here

CREATE TABLE IF NOT EXISTS slots (
    rev_id  INTEGER
                REFERENCES revisions(id)
                    ON DELETE RESTRICT,
    role_id INTEGER
                REFERENCES slot_roles(id)
                    ON DELETE NO ACTION,
    content_id INTEGER
                REFERENCES contents(id)
                    ON DELETE RESTRICT,

    PRIMARY KEY (rev_id, role_id)
);
