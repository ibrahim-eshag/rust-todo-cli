mod file_manager;
mod todo;
mod cli;

use clap::Parser;
use crate::file_manager::{list_all_todos, save_to_file, show_todo_description};
use todo::{Status, Todo,};
use crate::cli::{Action, Args};

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    match args.command {
        Some(Action::Create { title, description }) => {
            if title.len()==0 || description.len() ==0 {
                eprintln!("Error: enter valid arguments, e.g: todos create <title> <description>");
            }

            let mut todo = Todo::new(&title,&description);
            todo.normalize_title();
            match save_to_file(todo) {
                Ok(())=>println!("Todo Successfully added"),
                Err(e) => eprintln!("Failed to add entry: {:?}", e),
            }
        }
        Some(Action::List) => {
            println!("Listing all todos. \n");
            list_all_todos();
        }
        Some(Action::Show { title }) => {
            match title {
                Some(title)=>{
                   match  show_todo_description(&title){
                       Ok(description)=>println!("{}", description),
                       Err(e)=>eprintln!("Failed to show todo: {:?}", e),
                   }
                }
                None=>{
                    println!("No command received, please use --help for more info");
                }
            }
        }
        Some(Action::Archive { title }) => {
            match title {
                Some(title)=> {
                    match file_manager::move_to_archive(&title){
                        Ok(())=>println!("{} archived successfully.", title),
                        Err(e)=>eprintln!("Failed to archive todo: {:?}", e),
                    }
                }
                None=> {
                    println!("No command received, please use --help for more info");
                }
            }
        }
        Some(Action::Remove { title }) => {
            match title {
                Some(title)=> {
                    match file_manager::remove_todo(&title){
                        Ok(())=>println!("{} removed successfully.", title),
                        Err(e)=>eprintln!("Failed to remove todo: {:?}", e),
                    }
                }
                None=> {
                    println!("No command received, please use --help for more info");
                }
            }
        }
        None=> {
            println!("No command received, please use --help for more info");
        }
        _ => {}
    }

    // println!("Hello, world!");
    // let mut my_todo = Todo::new("second todo", "this todo is for description");
    //
    // my_todo.normalize_title();
    // my_todo.change_status(Status::Done);
    // println!("todo after being normalized: {:?}", my_todo);
    //
    // match save_to_file(my_todo) {
    //     Ok(())=>println!("Todo Successfully added"),
    //     Err(e) => eprintln!("Failed to add entry: {:?}", e),
    // }

    Ok(())
}

