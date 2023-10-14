use clap::{Args, command, Parser, Subcommand};

#[derive(Args, Debug)]
pub struct CreateTodo {
    #[arg(short, long, help = "The title of the todo")]
    pub title: String,
    #[arg(short, long, help = "The date the todo will be completed at, e.g. 1 day ago or 3 weeks later")]
    pub will_be_completed_at: String,

    #[arg(short, long, help = "The description of the todo")]
    pub description: Option<String>,
}

#[derive(Debug, Args)]
pub struct DeleteTodo {
    #[arg(short, long, help = "The id of the todo to delete")]
    pub id: i32,
}

#[derive(Debug, Args)]
pub struct CompleteTodo {
    #[arg(short, long, help = "The id of the todo to complete")]
    pub id: i32,
    #[arg(short, long, help = "The number of days ago the todo was completed, e.g. 1 day ago or 3 weeks later")]
    pub days_ago: String,
}

#[derive(Debug, Args)]
pub struct UpdateTodo {
    #[arg(short, long, help = "The id of the todo to update")]
    pub id: i32,
    #[arg(short, long, help = "The new title of the todo")]
    pub title: String,
    #[arg(short, long, help = "The new description of the todo")]
    pub description: String,
}

#[derive(Debug, Args)]
pub struct ShowTodo {
    #[arg(short = 'f', long, default_value_t = false, help = "Show fancy formatted todos")]
    pub is_fancy: bool,

    #[arg(short = 'l', long, help = "Limit the number of todos to show")]
    pub limit: Option<i64>,

    #[arg(short = 'i', long, help = "Show a specific todo item by id")]
    pub id: Option<i32>,
}

#[derive(Debug, Subcommand)]
pub enum TodoSubcommand {
    #[command(about = "Create a new todo")]
    Create(CreateTodo),
    #[command(about = "Update an existing todo")]
    Update(UpdateTodo),
    #[command(about = "Delete an existing todo")]
    Delete(DeleteTodo),
    #[command(about = "Complete an existing todo")]
    Complete(CompleteTodo),
    #[command(about = "Show all todos")]
    Show(ShowTodo),
}

#[derive(Debug, Parser)]
#[command(author="bishwas-py", version="0.0.1", about="Donow - A todo app written in Rust")]
pub struct TodoCommand {
    #[clap(subcommand)]
    pub command: TodoSubcommand,
}
