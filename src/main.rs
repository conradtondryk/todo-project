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
    Remove { name: String },
    View,
}

#[derive(Deserialize, Serialize, Debug)]
struct Task {
    id: i32,
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
                id: tasks.len() as i32 + 1,
                name: name.clone(),
                completed: false,
            };
            tasks.push(data);
            let file = File::create("list.json")?;
            serde_json::to_writer(file, &tasks)?;
            Ok(())
        }
        Commands::Remove { name } => {
            println!("{name} removed!");
            let index = tasks.iter().position(|task| task.name == name).unwrap();
            tasks.remove(index);
            let file = File::create("list.json")?;
            serde_json::to_writer(file, &tasks)?;
            Ok(())
        }
        Commands::View => {
            for task in tasks.iter() {
                println!("{:?}", task.name);
                println!("{:?}", task.completed);
            }
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
