use crate::TimeFormat;
use crate::Viewer;
use chrono::{DateTime, Local};
use dirs::home_dir;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use std::path::PathBuf;
// use cron::Schedule;
// use chrono::{DateTime, Datelike, Duration, Local, Timelike};
// use notify_rust::Notification;
// use std::str::FromStr;
// use std::thread;

const DB_FILE: &str = ".todos";

#[derive(Serialize, Deserialize)]
pub enum Status {
    NotStarted,
    InProgress,
    Completed,
}

#[derive(Serialize, Deserialize)]
pub struct Todo {
    pub name: String,
    pub status: Status,
    pub deadline: DateTime<Local>,
}

impl Todo {
    pub fn print(&self, viewer: &Viewer) -> String {
        let x = match viewer.time_format {
            TimeFormat::Absolute => self.deadline.format("%d %b ⋅ %H:%M").to_string(),
            TimeFormat::Remaining => {
                let now = Local::now();
                let duration = self.deadline.signed_duration_since(now).abs();
                if duration.num_days() > 0 {
                    format!("{}d", duration.num_days())
                } else if duration.num_hours() > 0 {
                    format!("{}h", duration.num_hours())
                } else if duration.num_minutes() > 0 {
                    format!("{}m", duration.num_minutes())
                } else {
                    format!("{}s", duration.num_seconds())
                }
            }
        };

        format!(
            "[{}] ({}) {}",
            match self.status {
                Status::NotStarted => " ",
                Status::InProgress => ".",
                Status::Completed => "✓",
            },
            x,
            self.name
        )
    }
}

fn get_db_path() -> PathBuf {
    home_dir()
        .expect("Could not find home directory")
        .join(DB_FILE)
}

pub fn save_todos(todos: &Vec<Todo>) -> io::Result<()> {
    let db_path = get_db_path();
    let json = serde_json::to_string(todos)?;
    fs::write(&db_path, json)?;
    Ok(())
}

pub fn load_todos() -> io::Result<Vec<Todo>> {
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

// pub fn schedule_todo_notification(
//     todo: Todo,
// ) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//     if todo.completed {
//         return Ok(());
//     }

//     let cron_expression = format!(
//         "{} {} {} {} *",
//         todo.deadline.minute(),
//         todo.deadline.hour(),
//         todo.deadline.day(),
//         todo.deadline.month()
//     );

//     let schedule = Schedule::from_str(&cron_expression)?;
//     let todo_name = todo.name.clone();

//     thread::spawn(move || {
//         for datetime in schedule.upcoming(Local) {
//             if datetime > todo.deadline {
//                 break;
//             }

//             // Sleep until next scheduled time
//             let now = Local::now();
//             if datetime > now {
//                 let duration =
//                     std::time::Duration::from_secs((datetime - now).num_seconds() as u64);
//                 thread::sleep(duration);
//             }

//             // Send notification
//             Notification::new()
//                 .summary("Kwik Deadline")
//                 .body(&format!("[task] {}", todo_name))
//                 .timeout(std::time::Duration::from_secs(10))
//                 .show()?;
//         }
//         Ok::<(), Box<dyn std::error::Error + Send + Sync + 'static>>(())
//     });

//     Ok(())
// }
