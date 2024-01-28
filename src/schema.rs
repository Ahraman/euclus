// @generated automatically by Diesel CLI.

diesel::table! {
    texts (id) {
        id -> Int4,
        body -> Text,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        display_name -> Varchar,
        pass -> Bytea,
        email -> Bytea,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    texts,
    users,
);
