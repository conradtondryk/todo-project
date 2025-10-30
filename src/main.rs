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
    Add {
        name: String,
    },
    Remove {
        name: String,
    },
    View {
        #[command(subcommand)]
        history: Option<ViewSubcommand>,
    },
    Complete {
        name: String,
    },
}

#[derive(Subcommand)]
enum ViewSubcommand {
    History,
}

#[derive(Deserialize, Serialize, Debug)]
struct Task {
    name: String,
    completed: bool,
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
        Commands::Remove { name } => {
            let index = tasks
                .iter()
                .position(|task| task.name == name)
                .ok_or_else(|| format!("Task '{name}' not found!"))?;
            tasks.remove(index);
            println!("{name} removed!");

            serde_json::to_writer(File::create("list.json")?, &tasks)?;
            Ok(())
        }
        Commands::View { history } => match history {
            Some(ViewSubcommand::History) => {
                for task in tasks.iter() {
                    if task.completed {
                        println!("{:?}", task.name)
                    }
                }
                Ok(())
            }
            None => {
                for task in tasks.iter() {
                    if !task.completed {
                        println!("{:?}", task.name)
                    }
                }
                Ok(())
            }
        },
        Commands::Complete { name } => {
            let index = tasks
                .iter()
                .position(|task| task.name == name)
                .ok_or_else(|| format!("Task '{name}' not found!"))?;
            tasks[index].completed = true;
            println!("'{name}' completed!");
            serde_json::to_writer(File::create("list.json")?, &tasks)?;
            Ok(())
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let Cli { command } = Cli::parse();
    json_editor(command)?;
    Ok(())
}
