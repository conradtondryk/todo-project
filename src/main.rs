#![warn(clippy::pedantic)]
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};

use std::{
    fs::{self, File},
    path::Path,
};

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
struct Task {
    name: String,
    completed: bool,
}

struct _TaskList {
    finished: Vec<Task>,
    unfinished: Vec<Task>,
}

fn json_editor(command: Commands) -> Result<(), Box<dyn std::error::Error>> {
    let json_data = fs::read_to_string(Path::new("list.json"))?;
    let mut tasks: Vec<Task> = serde_json::from_str(&json_data)?;

    match command {
        Commands::Add { name } => {
            println!("{name} added!");
            let data = Task {
                name: name.clone(),
                completed: false,
            };
            tasks.push(data);
            serde_json::to_writer(File::create("list.json")?, &tasks)?;
            Ok(())
        }
        Commands::Remove { id } => {
            let removed = tasks.remove(id - 1);
            println!("{} removed! ({})", id, removed.name);
            serde_json::to_writer(File::create("list.json")?, &tasks)?;
            Ok(())
        }
        Commands::View => {
            for (i, task) in tasks.iter().enumerate() {
                if !task.completed {
                    println!("{}) {}", i + 1, task.name);
                }
            }
            Ok(())
        }
        Commands::Complete { name } => {
            let index = tasks
                .iter()
                .position(|task| task.name == name)
                .ok_or_else(|| format!("Task '{name}' not found!"))?;
            tasks.remove(index);
            println!("'{name}' completed!");
            serde_json::to_writer(File::create("list.json")?, &tasks)?;
            Ok(())
        }
    }
}

// add id for easier task removing
// id = tasks.len() + 1
// need a way to not count the completed tasks
// logic for id shifting when complete tasks if tasks.id > task.id { let task.id = task.id - 1}
// add timestamps

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let Cli { command } = Cli::parse();
    json_editor(command)?;
    Ok(())
}
