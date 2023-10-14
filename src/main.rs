use crate::args::todo::{TodoCommand, TodoSubcommand};
use crate::ops::todo_ops::*;
use clap::Parser;

mod schema;
mod ops;
mod core;
mod args;

fn main() {
    let args = TodoCommand::parse();

    match args.command {
        TodoSubcommand::Create(todo) => {
            create_todo(todo);
        }
        TodoSubcommand::Update(todo) => {
            update_todo(todo);
        }
        TodoSubcommand::Delete(todo) => {
            delete_todo(todo);
        }
        TodoSubcommand::Show(show) => {
            show_todos(show);
        }
        TodoSubcommand::Complete(todo) => {
            complete_todo(todo)
        }
    }
}