use serde::Serialize;
use crate::db::schema::*;
use diesel::*;

#[derive(Debug, Identifiable, Queryable, Clone, Selectable, Serialize)]
#[diesel(table_name = zigs)]
pub struct Zig {
    pub id: String,
    pub user_name: String,
    pub button_counter: i32,
    pub ash_counter: i32
}

#[derive(Insertable)]
#[diesel(table_name = zigs)]
pub struct NewZig<'a> {
    pub id: &'a str,
    pub user_name: &'a str,
    pub button_counter: Option<i32>,
    pub ash_counter: Option<i32>,
}