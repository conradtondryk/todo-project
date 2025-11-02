#![warn(clippy::pedantic)]
use chrono::Local;
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::fs::{self, File};

#[derive(Parser)]
#[command(name = "todo")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add { name: String },
    Remove { id: usize },
    View,
    Complete { id: usize },
}

#[derive(Deserialize, Serialize, Debug)]
struct Task {
    timestamp: String,
    name: String,
}

fn load_tasks() -> Result<Vec<Task>, Box<dyn std::error::Error>> {
    match fs::read_to_string("list.json") {
        Ok(data) => serde_json::from_str(&data).map_err(Into::into),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(vec![]),
        Err(e) => Err(e.into()),
    }
}

fn save_tasks(tasks: &[Task]) -> Result<(), Box<dyn std::error::Error>> {
    serde_json::to_writer(File::create("list.json")?, tasks)?;
    Ok(())
}

impl Commands {
    fn add_task(tasks: &mut Vec<Task>, name: &str) -> Result<(), Box<dyn std::error::Error>> {
        if name.is_empty() {
            return Err("Name cannot be empty.".into());
        }
        tasks.push(Task {
            name: name.to_string(),
            timestamp: Local::now().format("%d/%m/%Y %H:%M").to_string(),
        });
        println!("{name} added!");
        Ok(())
    }
    fn remove_task(tasks: &mut Vec<Task>, id: usize) -> Result<(), Box<dyn std::error::Error>> {
        if id == 0 {
            return Err("ID cannot be 0.".into());
        } else if id > tasks.len() {
            return Err("ID is out of range.".into());
        }
        let removed = tasks.remove(id - 1);
        println!("{} removed! ({})", id, removed.name);
        Ok(())
    }
    fn view_tasks(tasks: &mut [Task]) {
        tasks.iter().enumerate().for_each(|(i, task)| {
            println!("{} {}) {}", task.timestamp, i + 1, task.name);
        });
    }
    fn task_complete(tasks: &mut Vec<Task>, id: usize) -> Result<(), Box<dyn std::error::Error>> {
        if id == 0 {
            return Err("ID cannot be 0.".into());
        } else if id > tasks.len() {
            return Err("ID is out of range.".into());
        }
        tasks.remove(id - 1);
        println!("'{id}' completed!");
        Ok(())
    }
}

fn json_editor(command: Commands) -> Result<(), Box<dyn std::error::Error>> {
    let mut tasks: Vec<Task> = load_tasks()?;

    match command {
        Commands::Add { name } => {
            Commands::add_task(&mut tasks, &name)?;
            save_tasks(&tasks)?;
        }
        Commands::Remove { id } => {
            Commands::remove_task(&mut tasks, id)?;
            save_tasks(&tasks)?;
        }
        Commands::View => {
            Commands::view_tasks(&mut tasks);
        }
        Commands::Complete { id } => {
            Commands::task_complete(&mut tasks, id)?;
            save_tasks(&tasks)?;
        }
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let Cli { command } = Cli::parse();
    json_editor(command)?;
    Ok(())
}
