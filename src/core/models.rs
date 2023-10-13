use chrono::{NaiveDateTime, Utc};
use diesel::{AsChangeset, Identifiable, Insertable, PgConnection, Queryable, RunQueryDsl};
use crate::schema::todos;

#[derive(Insertable)]
#[diesel(table_name = todos)]
pub struct NewTodo<'a> {
    pub title: &'a str,
    pub will_be_completed_at: NaiveDateTime,
    pub description: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

pub struct NewTodoEntry<'q> {
    pub title: &'q str,
    pub description: Option<String>,
    pub will_be_completed_at: NaiveDateTime,
}

impl NewTodo<'_> {
    pub fn create(self, connection: &mut PgConnection) -> usize {
        use crate::schema::todos::dsl::*;

        diesel::insert_into(todos)
            .values(&self)
            .execute(connection)
            .expect("Error saving new todo")
    }

    pub fn new(entry: NewTodoEntry) -> NewTodo {
        NewTodo {
            title: entry.title,
            description: entry.description,
            will_be_completed_at: entry.will_be_completed_at,
            created_at: Some(Utc::now().naive_local()),
            updated_at: Some(Utc::now().naive_local()),
        }
    }
}

#[derive(Debug, Queryable, AsChangeset, Identifiable)]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub will_be_completed_at: Option<NaiveDateTime>,
    pub completed_at: Option<NaiveDateTime>,
}

impl Todo {
    pub fn display_todos_fancy(&self) {
        let description = match &self.description {
            Some(desc) => desc,
            None => "<no description>"
        };

        let created_at = match &self.created_at {
            Some(time) => format!("{}", time.format("%Y-%m-%d %H:%M:%S")),
            None => "<no creation time>".to_string()
        };

        let updated_at = match &self.updated_at {
            Some(time) => format!("{}", time.format("%Y-%m-%d %H:%M:%S")),
            None => "<no update time>".to_string()
        };

        let completion_time = match &self.will_be_completed_at {
            Some(time) => format!("{}", time.format("%Y-%m-%d %H:%M:%S")),
            None => "<no completion time>".to_string()
        };

        println!("Todo #{}\n\
                  Title: {}\n\
                  Description: {}\n\
                  Created At: {}\n\
                  Updated At: {}\n\
                  Completion Time: {}\n\
                  ========================================",
                 self.id,
                 self.title,
                 description,
                 created_at,
                 updated_at,
                 completion_time);
    }
}