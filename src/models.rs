use diesel::{Queryable, Selectable};
use time::OffsetDateTime;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub username: String,
    pub display_name: String,
    pub pass_word: Vec<u8>,
    pub email: Vec<u8>,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::texts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Text {
    pub id: i32,
    pub body: String,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::contents)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Content {
    pub id: i32,
    pub text_id: i32,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::slot_roles)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct SlotRole {
    pub id: i32,
    pub title: String,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::revisions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Revision {
    pub id: i32,
    pub page_id: i32,
    pub parent_id: Option<i32>,
    pub submit_time: OffsetDateTime,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::slots)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Slot {
    pub rev_id: i32,
    pub role_id: i32,
    pub content_id: Option<i32>,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::pages)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Page {
    pub id: i32,
    pub title: String,
    pub rev_id: i32,
    pub last_touched: OffsetDateTime,
}
