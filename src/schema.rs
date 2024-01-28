// @generated automatically by Diesel CLI.

diesel::table! {
    contents (id) {
        id -> Int4,
        text_id -> Int4,
    }
}

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
    revisions (id) {
        id -> Int4,
        page_id -> Nullable<Int4>,
        user_id -> Nullable<Int4>,
        prev_rev -> Nullable<Int4>,
        creation_time -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    slot_roles (id) {
        id -> Int4,
        #[max_length = 64]
        title -> Varchar,
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
        pass_word -> Bytea,
        email -> Bytea,
    }
}

diesel::joinable!(contents -> texts (text_id));
diesel::joinable!(revisions -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    contents,
    pages,
    revisions,
    slot_roles,
    texts,
    users,
);
