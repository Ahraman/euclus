// @generated automatically by Diesel CLI.

diesel::table! {
    pages (id) {
        id -> Int4,
        #[max_length = 512]
        title -> Varchar,
        cur_rev -> Int4,
        last_touched -> Timestamptz,
    }
}

diesel::table! {
    texts (id) {
        id -> Int4,
        body -> Text,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 255]
        username -> Varchar,
        #[max_length = 255]
        display_name -> Varchar,
        pass -> Bytea,
        email -> Bytea,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    pages,
    texts,
    users,
);
