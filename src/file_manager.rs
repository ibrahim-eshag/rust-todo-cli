use crate::todo::{Status, Todo};
use std::fs;
use std::io;
use std::path::Path;

// TODO: this one better to be moved to TODO struct
pub fn save_to_file(todo: Todo)-> io::Result<()> {
    let file_name = match todo.status {
        Status::Pending => format!("todos_data/{}.md", todo.title),
        Status::Done => format!("todos_data/.archive/{}.done.md", todo.title),
    };

    println!("Saving to file: {}", file_name);
    save_file(&file_name, &todo.description)
}

// it's more convenient to remove the directory, but took the chance to learn how to remove files
pub fn remove_all_done_todos() {
    let path = "todos_data/.archive";
    let files = fs::read_dir(path).unwrap();
    for file in files {
        let file = file.unwrap();
        let file_name = file.path();
        remove_file(file_name.to_str().unwrap());
    }
}

pub fn list_all_todos() {
    let path = "todos_data";
    list_dir_files(path, false);

    let path = "todos_data/.archive";
    list_dir_files(path, true);
}

pub fn show_todo_description(title: &str) -> io::Result<String> {
    let path = format!("todos_data/{}.md", title);
    read_file(&path)
}

fn list_dir_files(path: &str, is_archived:bool) {
    let files = fs::read_dir(path).unwrap();
    if is_archived {
        println!("./.archive");
    }
    for file in files {
        let file = file.unwrap();
        let file_name = file.file_name();
        // if archived, indent the file name
        if is_archived {
            println!("    {}", file_name.to_str().unwrap());
        }else {
        println!("{}", file_name.to_str().unwrap());
        }
    }
}

fn read_file(path: &str) -> io::Result<String> {
    let content = fs::read_to_string(path)?;
    Ok(content)
}

// TODO: better to keep it generic by passing array of paths
pub fn remove_all_todos() {
    let path = "todos_data";
    remove_dir(path);

    let path = "todos_data/.archive";
    remove_dir(path);
}

fn save_file(path: &str, content: &str) -> io::Result<()> {
    // Extract parent directory and create it
    if let Some(parent) = Path::new(path).parent() {
        // Check if parent exists as a file
        if parent.exists() && !parent.is_dir() {
            return Err(io::Error::new(
                io::ErrorKind::AlreadyExists,
                format!("Path '{}' exists but is not a directory", parent.display())
            ));
        }
        fs::create_dir_all(parent)?;
    }

    fs::write(path, content)?;
    println!("Content successfully saved to '{}'", path);
    Ok(())
}

fn remove_file(path: &str) -> io::Result<()> {
    fs::remove_file(path)?;
    println!("Content successfully removed from '{}'", path);
    Ok(())
}

fn remove_dir(path: &str) -> io::Result<()> {
    fs::remove_dir(path)?;
    println!("Content successfully removed from '{}'", path);
    Ok(())
}
