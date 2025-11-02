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
}

#[derive(Deserialize, Serialize, Debug)]
struct Task {
    timestamp: String,
    name: String,
}

#[derive(Deserialize, Serialize)]
#[serde(transparent)]
struct Tasks(Vec<Task>);

impl Tasks {
    fn load() -> Result<Self, Box<dyn std::error::Error>> {
        match fs::read_to_string("list.json") {
            Ok(data) => serde_json::from_str(&data).map_err(Into::into),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(Tasks(vec![])),
            Err(e) => Err(e.into()),
        }
    }

    fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        serde_json::to_writer(File::create("list.json")?, &self.0)?;
        Ok(())
    }

    fn add(&mut self, name: &str) -> Result<(), Box<dyn std::error::Error>> {
        if name.is_empty() {
            return Err("Name cannot be empty.".into());
        }
        self.0.push(Task {
            name: name.to_string(),
            timestamp: Local::now().format("%d/%m/%Y %H:%M").to_string(),
        });
        println!("{name} added!");
        Ok(())
    }

    fn remove(&mut self, id: usize) -> Result<(), Box<dyn std::error::Error>> {
        if id == 0 {
            return Err("ID cannot be 0.".into());
        } else if id > self.0.len() {
            return Err("ID is out of range.".into());
        }
        let removed = self.0.remove(id - 1);
        println!("{} removed! ({})", id, removed.name);
        Ok(())
    }

    fn view(&self) {
        self.0.iter().enumerate().for_each(|(i, task)| {
            println!("{} {}) {}", task.timestamp, i + 1, task.name);
        });
    }
}

fn json_editor(command: Commands) -> Result<(), Box<dyn std::error::Error>> {
    let mut tasks = Tasks::load()?;

    match command {
        Commands::Add { name } => {
            tasks.add(&name)?;
            tasks.save()?;
        }
        Commands::Remove { id } => {
            tasks.remove(id)?;
            tasks.save()?;
        }
        Commands::View => {
            tasks.view();
        }
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let Cli { command } = Cli::parse();
    json_editor(command)?;
    Ok(())
}
