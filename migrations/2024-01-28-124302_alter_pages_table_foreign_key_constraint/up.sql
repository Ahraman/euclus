-- Your SQL goes here

ALTER TABLE IF EXISTS pages
        ADD CONSTRAINT fk_cur_rev FOREIGN KEY (cur_rev)
                REFERENCES revisions(id);
