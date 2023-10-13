use chrono::NaiveDateTime;
use diesel::{AsChangeset, Identifiable, Insertable, Queryable};
use crate::schema::todos;

#[derive(Insertable)]
#[diesel(table_name = todos)]
pub struct NewTodo<'a> {
    pub title: &'a str,
    pub description: &'a str,
    // 'a are lifetime annotations
}

#[derive(Debug, Queryable, AsChangeset, Identifiable)]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub will_be_completed_at: Option<NaiveDateTime>,
    pub completed_at: Option<NaiveDateTime>
}