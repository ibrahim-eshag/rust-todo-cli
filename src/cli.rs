use clap::Parser;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[clap(subcommand)]
    pub command: Option<Action>,
    /// Creates or modifies todo list at specified path
    #[clap(short, long, default_value = "./todos_data/")]
    pub todos_dir: String,
}

#[derive(clap::Subcommand)]
pub enum Action {
    #[clap(about = "create new todo")]
    Create {
        #[clap(help = "Title of the todo")]
        title: String,
        #[clap(help = "Description of the todo")]
        description: String,
    },
    #[clap(about = "List all todos")]
    List,
    #[clap(about = "Show todo description")]
    Show { title: Option<String> },
    #[clap(about = "Archive a todo item")]
    Archive { title: Option<String> },
    #[clap(about = "Removes entry with id from todo list")]
    Remove { title: Option<String> },
    #[clap(about = "Removes all entries from todo list")]
    Clear,
    #[clap(about = "Marks item with title as done on todo list")]
    Done { title: Option<String> },
    #[clap(about = "undo a todo item")]
    Undo { title: Option<String> },
}
