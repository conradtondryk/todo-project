#![warn(clippy::pedantic)]
use anyhow::{Result, bail};
use chrono::{DateTime, Local};
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::ErrorKind;

#[derive(Parser)]
#[command(name = "todo")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Add { name: String },
    Remove { id: usize },
    View,
}

impl Command {
    fn handle(self) -> Result<()> {
        let mut tasks = Tasks::load()?;

        match self {
            Command::Add { name } => {
                tasks.add(&name)?;
                tasks.save()?;
            }
            Command::Remove { id } => {
                tasks.remove(id)?;
                tasks.save()?;
            }
            Command::View => {
                tasks.view();
            }
        }
        Ok(())
    }
}

#[derive(Deserialize, Serialize, Debug)]
struct Task {
    timestamp: DateTime<Local>,
    name: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(transparent)]
struct Tasks(Vec<Task>);

impl Tasks {
    fn load() -> Result<Self> {
        match fs::read_to_string("list.json") {
            Ok(data) => serde_json::from_str(&data).map_err(Into::into),
            Err(e) if e.kind() == ErrorKind::NotFound => Ok(Tasks(vec![])),
            Err(e) => Err(e.into()),
        }
    }

    fn save(&self) -> Result<()> {
        serde_json::to_writer(File::create("list.json")?, &self.0)?;
        Ok(())
    }

    fn add(&mut self, name: &str) -> Result<()> {
        if name.is_empty() {
            bail!("Name cannot be empty.");
        }
        self.0.push(Task {
            name: name.to_string(),
            timestamp: Local::now(),
        });
        println!("{name} added!");
        Ok(())
    }
    fn remove(&mut self, id: usize) -> Result<()> {
        if id == 0 {
            bail!("ID cannot be 0.");
        } else if id > self.0.len() {
            bail!("ID is out of range.");
        }
        let removed = self.0.remove(id - 1);
        println!("{} removed! ({})", id, removed.name);
        Ok(())
    }
    fn view(&self) {
        self.0.iter().enumerate().for_each(|(i, task)| {
            println!(
                "{} ({}) {}",
                task.timestamp.format("%d/%m/%Y %H:%M"),
                i + 1,
                task.name
            );
        });
    }
}

fn main() -> Result<()> {
    let Cli { command } = Cli::parse();
    command.handle()?;
    Ok(())
}
