mod todo;
use chrono::{Local, TimeZone};
use colored::*;
use ctrlc;
use lazy_static::lazy_static;
use regex::Regex;
use std::io::Write;
use std::process;
use todo::{load_todos, save_todos, Status, Todo};

enum TimeFormat {
    Absolute,
    Remaining,
}

struct Viewer {
    time_format: TimeFormat,
}

impl Viewer {
    fn new() -> Self {
        Self {
            time_format: TimeFormat::Remaining,
        }
    }

    fn flip_time_format(&mut self) {
        self.time_format = match self.time_format {
            TimeFormat::Absolute => TimeFormat::Remaining,
            TimeFormat::Remaining => TimeFormat::Absolute,
        };
    }
}

lazy_static! {
    static ref DELETE_RE: Regex = Regex::new(r"^d\s+(?P<index>\d+)$").unwrap();
    static ref TOGGLE_RE: Regex = Regex::new(r"^t\s+(?P<index>\d+)$").unwrap();
    static ref ADD_RE: Regex = Regex::new(r"^a\s+\((?P<datetime>[^)]+)\)\s+(?P<name>.+)$").unwrap();
    static ref EDIT_RE: Regex = Regex::new(r"^e (?P<index>\d+) (?P<new_name>.+)$").unwrap();
}

fn process_command(input: &str, todos: &mut Vec<Todo>, viewer: &mut Viewer) -> Result<(), String> {
    // short commands
    if input.trim().len() == 1 {
        match input.trim() {
            "q" => process::exit(0),
            "s" => {
                viewer.flip_time_format();
            }
            _ => {}
        };
    }

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
            let new_status = match todos[index].status {
                Status::NotStarted => Status::InProgress,
                Status::InProgress => Status::Completed,
                Status::Completed => Status::NotStarted,
            };
            todos[index].status = new_status;
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
            status: Status::NotStarted,
            deadline,
        });
        Ok(())
    } else {
        Err("Invalid command format".to_string())
    }
}

fn show_todos(todos: &Vec<Todo>, viewer: &Viewer) {
    print!("\x1B[2J\x1B[1;1H");

    let mut x = 0;
    let current = Local::now();

    for todo in todos {
        let t = todo.print(viewer);
        let mut z = match todo.status {
            Status::NotStarted => t.white(),
            Status::InProgress => t.yellow(),
            Status::Completed => t.green(),
        };
        if todo.deadline < current {
            z = z.red();
        }

        println!("{}.\t{}", x, z);
        x += 1;
    }

    print!("\n--------------------------------\n> ");
    std::io::stdout().flush().unwrap();
}

fn parse_instruction(todos: &mut Vec<Todo>, viewer: &mut Viewer) {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();

    match process_command(input, todos, viewer) {
        Ok(_) => (),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    ctrlc::set_handler(move || {
        process::exit(0);
    })
    .unwrap();

    let mut viewer = Viewer::new();
    let mut todos: Vec<Todo> = Vec::new();
    match load_todos() {
        Ok(t) => todos = t,
        Err(_) => (),
    }

    loop {
        todos.sort_by(|a, b| a.deadline.cmp(&b.deadline));
        save_todos(&todos).unwrap();
        show_todos(&todos, &viewer);
        parse_instruction(&mut todos, &mut viewer);
    }
}
