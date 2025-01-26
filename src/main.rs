use chrono::{DateTime, Local, TimeZone};
use colored::*;
use dirs::home_dir;
use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use std::io::Write;
use std::path::PathBuf;

const DB_FILE: &str = ".todos";

lazy_static! {
    static ref DELETE_RE: Regex = Regex::new(r"^d\s+(?P<index>\d+)$").unwrap();
    static ref TOGGLE_RE: Regex = Regex::new(r"^t\s+(?P<index>\d+)$").unwrap();
    static ref ADD_RE: Regex = Regex::new(r"^a\s+\((?P<datetime>[^)]+)\)\s+(?P<name>.+)$").unwrap();
    static ref EDIT_RE: Regex = Regex::new(r"^e (?P<index>\d+) (?P<new_name>.+)$").unwrap();
}

#[derive(Serialize, Deserialize)]
struct Todo {
    name: String,
    completed: bool,
    deadline: DateTime<Local>,
}

impl std::fmt::Display for Todo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let formatted_date = self.deadline.format("%d %b ⋅ %H:%M");
        write!(
            f,
            "[{}] ({}) {}",
            if self.completed { "✓" } else { " " },
            formatted_date,
            self.name
        )
    }
}

fn get_db_path() -> PathBuf {
    home_dir()
        .expect("Could not find home directory")
        .join(DB_FILE)
}

fn save_todos(todos: &Vec<Todo>) -> io::Result<()> {
    let db_path = get_db_path();
    let json = serde_json::to_string(todos)?;
    fs::write(&db_path, json)?;
    Ok(())
}

fn load_todos() -> io::Result<Vec<Todo>> {
    let db_path = get_db_path();
    if !db_path.exists() {
        fs::write(&db_path, "[]")?;
        return Ok(Vec::new());
    }

    let file_contents = fs::read_to_string(&db_path)?;
    if file_contents.trim().is_empty() {
        return Ok(Vec::new());
    }
    match serde_json::from_str(&file_contents) {
        Ok(todos) => Ok(todos),
        Err(e) => Err(io::Error::new(io::ErrorKind::InvalidData, e)),
    }
}

fn process_command(input: &str, todos: &mut Vec<Todo>) -> Result<(), String> {
    if let Some(caps) = DELETE_RE.captures(input) {
        let index = caps["index"]
            .parse::<usize>()
            .map_err(|_| "Invalid index format")?;

        if index < todos.len() {
            todos.remove(index);
            Ok(())
        } else {
            Err("Index out of bounds".to_string())
        }
    } else if let Some(caps) = TOGGLE_RE.captures(input) {
        let index = caps["index"]
            .parse::<usize>()
            .map_err(|_| "Invalid index format")?;

        if index < todos.len() {
            todos[index].completed = !todos[index].completed;
            Ok(())
        } else {
            Err("Index out of bounds".to_string())
        }
    } else if let Some(caps) = EDIT_RE.captures(input) {
        let index = caps["index"]
            .parse::<usize>()
            .map_err(|_| "Invalid index format")?;

        if index < todos.len() {
            todos[index].name = caps["new_name"].trim().to_string();
            Ok(())
        } else {
            Err("Index out of bounds".to_string())
        }
    } else if let Some(caps) = ADD_RE.captures(input) {
        let datetime_str = format!("{} {}", caps["datetime"].trim(), Local::now().format("%Y"));
        let naive_datetime = chrono::NaiveDateTime::parse_from_str(&datetime_str, "%d %b %H:%M %Y")
            .map_err(|_| "Invalid datetime format")?;
        let deadline = Local.from_local_datetime(&naive_datetime).single().unwrap();

        todos.push(Todo {
            name: caps["name"].trim().to_string(),
            completed: false,
            deadline,
        });
        Ok(())
    } else {
        Err("Invalid command format".to_string())
    }
}

fn show_todos(todos: &Vec<Todo>) {
    print!("\x1B[2J\x1B[1;1H");

    let current = Local::now();

    let mut x = 0;
    for todo in todos {
        let t = format!("{}", todo);
        if todo.completed {
            println!("{}. {}", x, t.green());
        } else if todo.deadline < current {
            println!("{}. {}", x, t.red());
        } else {
            println!("{}. {}", x, t);
        }
        x += 1;
    }

    print!("\n--------------------------------\n> ");
    std::io::stdout().flush().unwrap();
}

fn parse_instruction(todos: &mut Vec<Todo>) {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();

    match process_command(input, todos) {
        Ok(_) => (),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    let mut todos: Vec<Todo> = Vec::new();
    match load_todos() {
        Ok(t) => todos = t,
        Err(_) => (),
    }

    loop {
        todos.sort_by(|a, b| a.deadline.cmp(&b.deadline));
        save_todos(&todos).unwrap();
        show_todos(&todos);
        parse_instruction(&mut todos);
    }
}
