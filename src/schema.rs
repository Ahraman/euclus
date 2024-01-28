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
    slots (rev_id, role_id) {
        rev_id -> Int4,
        role_id -> Int4,
        content_id -> Nullable<Int4>,
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
diesel::joinable!(slots -> contents (content_id));
diesel::joinable!(slots -> revisions (rev_id));
diesel::joinable!(slots -> slot_roles (role_id));

diesel::allow_tables_to_appear_in_same_query!(
    contents,
    pages,
    revisions,
    slot_roles,
    slots,
    texts,
    users,
);
