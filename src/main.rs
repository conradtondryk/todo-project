#![warn(clippy::pedantic)]
use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};

use std::fs::{self, File};

const TASKS_FILE: &str = "list.json";

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
    Complete { name: String },
}

#[derive(Deserialize, Serialize, Debug)]
enum State {
    Pending,
    Completed,
}

#[derive(Deserialize, Serialize, Debug)]
struct Task {
    name: String,
    state: State,
}

struct _TaskList {
    finished: Vec<Task>,
    unfinished: Vec<Task>,
}

fn load_tasks() -> Result<Vec<Task>> {
    match fs::read_to_string(TASKS_FILE) {
        Ok(data) => Ok(serde_json::from_str(&data)?),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(vec![]),
        Err(e) => Err(e.into()),
    }
}

fn save_tasks(tasks: &[Task]) -> Result<()> {
    serde_json::to_writer(File::create(TASKS_FILE)?, tasks)?;
    Ok(())
}

fn json_editor(command: Commands) -> Result<()> {
    let mut tasks: Vec<Task> = load_tasks()?;

    match command {
        Commands::Add { name } => {
            tasks.push(Task {
                name: name.clone(),
                state: State::Pending,
            });
            println!("{name} added!");
            save_tasks(&tasks)?;
        }
        Commands::Remove { id } => {
            let removed = tasks.remove(id - 1);
            println!("{} removed! ({})", id, removed.name);
            save_tasks(&tasks)?;
        }
        Commands::View => {
            tasks.iter().enumerate().for_each(|(i, task)| {
                println!("{}) {}", i + 1, task.name);
            });
            return Ok(());
        }
        Commands::Complete { name } => {
            let task = tasks
                .iter_mut()
                .find(|task| task.name == name)
                .context(format!("Task '{name}' not found!"))?;
            task.state = State::Completed;
            println!("'{name}' completed!");
            save_tasks(&tasks)?;
        }
    }
    Ok(())
}

fn main() -> Result<()> {
    let Cli { command } = Cli::parse();
    json_editor(command)?;
    Ok(())
}
