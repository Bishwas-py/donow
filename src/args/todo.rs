use clap::{Args, Parser, Subcommand};

#[derive(Args, Debug)]
pub struct CreateTodo {
    pub title: String,
    pub description: String,
}

#[derive(Debug, Args)]
pub struct DeleteTodo {
    pub id: i32,
}

#[derive(Debug, Args)]
pub struct CompleteTodo {
    pub id: i32,
    pub days_ago: String
}

#[derive(Debug, Args)]
pub struct UpdateTodo {
    pub id: i32,
    pub title: String,
    pub description: String,
}

#[derive(Debug, Subcommand)]
pub enum TodoSubcommand {
    Create(CreateTodo),
    Update(UpdateTodo),
    Delete(DeleteTodo),
    Complete(CompleteTodo),
    Show,
}

#[derive(Debug, Parser)]
pub struct TodoCommand {
    #[clap(subcommand)]
    pub command: TodoSubcommand,
}