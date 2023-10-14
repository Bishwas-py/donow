use crate::args::todo::{CompleteTodo, CreateTodo, DeleteTodo, ShowTodo, UpdateTodo};
use crate::core::db::establish_connection;
use crate::core::models::{NewTodo, NewTodoEntry, Todo};
use diesel::prelude::*;
use crate::core::utils::parse_relative_time;

pub fn create_todo(todo: CreateTodo) -> usize {
    println!("Creating todo: {:?}", todo);

    let connection = &mut establish_connection();
    let will_be_completed_at = parse_relative_time(&todo.will_be_completed_at)
        .unwrap()
        .naive_utc();

    let todo_entry = NewTodoEntry {
        title: &todo.title,
        description: todo.description,
        will_be_completed_at,
    };

    let new_todo = NewTodo::new(todo_entry);
    new_todo.create(connection)
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

pub fn show_todos(show: ShowTodo) {
    println!("Showing todos");
    use crate::schema::todos::dsl::*;

    let connection = &mut establish_connection();

    let results = match show.id {
        None => {
            match show.limit {
                None => {
                    todos
                        .load::<Todo>(connection)
                }
                Some(limit_num) => {
                    todos.limit(limit_num)
                        .load::<Todo>(connection)
                }
            }
        }
        Some(todo_id) => {
            todos.filter(id.eq(todo_id))
                .load::<Todo>(connection)
        }
    }.expect("Error loading todos");

    println!("Displaying {} todos", results.len());
    if show.is_fancy && !results.is_empty() {
        println!("========================================");
        for todo in results {
            todo.display_todos_fancy()
        }
    } else {
        for todo in results {
            println!("{:?}", todo);
        }
    }
}