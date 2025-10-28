use clap::{Parser, Subcommand};
use serde::Deserialize;
use std::env;

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
    View { list: Vec<String> },
}

fn main() {
    let input = Cli::parse();

    match input.command {
        Commands::Add { name } => println!("{name} added!"),
        Commands::Remove { name } => println!("{name} removed!"),
        Commands::View { list } => print!("{:?}", list),
    }
}
