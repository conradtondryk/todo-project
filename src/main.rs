use clap::{Parser, Subcommand};
use serde::Deserialize;
use serde_json;
use std::{env, fs, path::Path};

#[derive(Parser)]
#[command(name = "todo")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add { name: String },
    Remove { name: String },
    View,
}

#[derive(Deserialize, Debug)]
struct Task {
    id: i32,
    name: String,
    completed: bool,
}

fn json_editor(command: Commands) -> Result<(), Box<dyn std::error::Error>> {
    let json_data = fs::read_to_string(Path::new("list.json")).expect("");
    let task: Task = serde_json::from_str(&json_data)?;

    match command {
        Commands::Add { name } => {
            println!("{name} added!");
            Ok(())
        }
        Commands::Remove { name } => {
            println!("{name} removed!");
            Ok(())
        }
        Commands::View => {
            println!("{:?}", task.id);
            println!("{:?}", task.name);
            println!("{:?}", task.completed);
            Ok(())
        }
    }
}

fn main() {
    let input = Cli::parse();

    let cmd = input.command;
    if let Err(e) = json_editor(cmd) {
        eprintln!("Error: {}", e);
    }
}
