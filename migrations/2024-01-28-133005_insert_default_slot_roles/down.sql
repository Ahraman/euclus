-- This file should undo anything in `up.sql`

DELETE FROM slot_roles
    WHERE title = 'main';
