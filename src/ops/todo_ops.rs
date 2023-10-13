use crate::args::todo::{CompleteTodo, CreateTodo, DeleteTodo, UpdateTodo};
use crate::db::establish_connection;
use crate::models::{NewTodo, Todo};
use diesel::prelude::*;
use crate::utils::parse_relative_time;

pub fn create_todo(todo: CreateTodo) -> usize {
    println!("Creating todo: {:?}", todo);
    use crate::schema::todos::dsl::*;

    let connection = &mut establish_connection();

    let new_todo = NewTodo {
        description: &todo.description,
        title: &todo.title,
    };

    diesel::insert_into(todos)
        .values(&new_todo)
        .execute(connection)
        .expect("Error saving new todo")
}


pub fn update_todo(todo: UpdateTodo) {
    println!("Updating todo: {:?}", todo);
    use crate::schema::todos::dsl::*;

    let connection = &mut establish_connection();
    diesel::update(todos.find(todo.id))
        .set((
            title.eq(todo.title),
            description.eq(todo.description),
        ))
        .execute(connection)
        .expect("Error updating todo");
}

pub fn delete_todo(todo: DeleteTodo) {
    println!("Deleting todo: {:?}", todo);
    use crate::schema::todos::dsl::*;

    let connection = &mut establish_connection();
    diesel::delete(todos.find(todo.id))
        .execute(connection)
        .expect("Error deleting todo");
}

pub fn complete_todo(todo: CompleteTodo) {
    println!("Completed todo: {:?}", todo);
    use crate::schema::todos::dsl::*;

    let connection = &mut establish_connection();

    let competed_at_date = parse_relative_time(&todo.days_ago).unwrap().naive_utc();
    diesel::update(todos.find(todo.id))
        .set((
            completed_at.eq(competed_at_date),
        ))
        .execute(connection)
        .expect("Error updating todo");
}

pub fn show_todos() {
    println!("Showing todos");
    use crate::schema::todos::dsl::*;

    let connection = &mut establish_connection();
    let results = todos
        .load::<Todo>(connection)
        .expect("Error loading todos");

    println!("Displaying {} todos", results.len());
    for todo in results {
        println!("{:?}", todo);
    }
}