-- This file should undo anything in `up.sql`

ALTER TABLE IF EXISTS pages
        DROP CONSTRAINT IF EXISTS fk_cur_rev;
