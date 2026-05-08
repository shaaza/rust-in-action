use std::path::PathBuf;

use clap::{Parser, Subcommand};

mod store;

use store::{KVStore, Store};

#[derive(Debug, Parser)]
#[command(name = "actionkv")]
struct Cli {
    filepath: PathBuf,

    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    Get { key: String },
    Insert { key: String, value: String },
    Update { key: String, value: String },
    Delete { key: String },
}

fn main() {
    let cli = Cli::parse();
    let mut store = KVStore::open(cli.filepath);

    match &cli.command {
        Command::Get { key } => {
            store.get(key);
        }
        Command::Insert { key, value } => {
            store.insert(key, value);
        }
        Command::Update { key, value } => {
            store.update(key, value);
        }
        Command::Delete { key } => {
            store.delete(key);
        }
    }
}
