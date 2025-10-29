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

fn json_editor() -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new("list.json");

    let json_data = fs::read_to_string(path).expect("");
    let user: Task = serde_json::from_str(&json_data)?;
    println!("{:?}", user.id);
    println!("{:?}", user.name);
    println!("{:?}", user.completed);
    Ok(())
}

fn main() {
    let input = Cli::parse();

    match input.command {
        Commands::Add { name } => println!("{name} added!"),
        Commands::Remove { name } => println!("{name} removed!"),
        Commands::View => json_editor().unwrap(),
    }
}
