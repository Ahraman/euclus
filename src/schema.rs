// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        display_name -> Varchar,
        pass -> Bytea,
        email -> Bytea,
    }
}
