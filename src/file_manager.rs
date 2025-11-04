use crate::todo::{Status, Todo};
use std::fs;
use std::io;
use std::path::Path;

// TODO: this one better to be moved to TODO struct
pub fn save_to_file(todo: Todo) -> io::Result<()> {
    let file_name = match todo.status {
        Status::Pending => format!("todos_data/{}.md", todo.title),
        Status::Done => format!("todos_data/.archive/{}.done.md", todo.title),
    };

    println!("Saving to file: {}", file_name);
    save_file(&file_name, &todo.description)
}

pub fn remove_todo(title: &str) -> io::Result<()> {
    let path = find_todo_path_by_title(title)?;
    remove_file(&path)
}
pub fn change_todo_status(title: &str, status: Status) -> io::Result<()> {
    let old_path = find_todo_path_by_title(title)?;

    // changing the status of a todo immediately move it to the home directory (even if it's in archive)
    let new_path = match status {
        Status::Done => {
            format!("todos_data/{}.done.md", title)
        }
        Status::Pending => {
            format!("todos_data/{}.todo.md", title)
        }
    };

    rename_file(&old_path, &new_path)?;
    Ok(())
}

fn rename_file(old_path: &str, new_path: &str) -> io::Result<()> {
    fs::rename(old_path, new_path)
}
fn find_todo_path_by_title(title: &str) -> io::Result<String> {
    let paths = [
        format!("todos_data/{}.todo.md", title),
        format!("todos_data/{}.done.md", title),
        format!("todos_data/.archive/{}.todo.md", title),
        format!("todos_data/.archive/{}.done.md", title),
    ];
    for p in paths {
        if Path::new(&p).exists() {
            return Ok(p);
        }
    }
    Err(io::Error::new(io::ErrorKind::NotFound, "Todo not found"))
}

pub fn list_all_todos() {
    let path = "todos_data";
    list_dir_files(path, false);

    let path = "todos_data/.archive";
    list_dir_files(path, true);
}

pub fn show_todo_description(title: &str) -> io::Result<String> {
    let path = find_todo_path_by_title(&title)?;
    read_file(&path)
}

pub fn move_to_archive(title: &str) -> io::Result<()> {
    let path = find_todo_path_by_title(&title)?;
    let status = get_todo_status(&path).to_string();
    println!("Moving to archive: {}", path);
    let path_archive = format!("todos_data/.archive/{}.{}.md", title,status);
    move_file(&path, &path_archive)
}

fn move_file(from: &str, to: &str) -> io::Result<()> {
    fs::rename(from, to)
}

fn list_dir_files(path: &str, is_archived: bool) {
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
        } else {
            println!("{}", file_name.to_str().unwrap());
        }
    }
}

fn read_file(path: &str) -> io::Result<String> {
    let content = fs::read_to_string(path)?;
    Ok(content)
}

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
                format!("Path '{}' exists but is not a directory", parent.display()),
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

fn get_todo_status(path: &str) -> Status {
    if path.contains(".done.md") {
        Status::Done
    } else {
        Status::Pending
    }
}

