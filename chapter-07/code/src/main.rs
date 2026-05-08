use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "actionkv")]
struct Cli {
    filepath: PathBuf,

    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    Get,
    Insert { key: String, value: String },
    Update { key: String, value: String },
    Delete { key: String },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Command::Get => println!("{:?}", cli.command),
        Command::Insert { .. } => {
            println!("{:?}", cli.command);
        }
        Command::Update { .. } => {
            println!("{:?}", cli.command);
        }
        Command::Delete { .. } => {
            println!("{:?}", cli.command);
        }
    }
}
