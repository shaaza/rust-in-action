use std::path::PathBuf;

use clap::{Parser, Subcommand};
use libactionkv::{KVStore, Store};

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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let Cli { filepath, command } = Cli::parse();
    let mut store = KVStore::open(filepath)?;

    match &command {
        Command::Get { key } => match store.get(key) {
            Some(value) => println!("{key}={value}"),
            None => println!("{key} not found in {:?}", store.filepath()),
        },
        Command::Insert { key, value } => {
            store.insert(key, value)?;
            println!("insert {key}={value} into {:?}", store.filepath());
        }
        Command::Update { key, value } => {
            store.update(key, value)?;
            println!("update {key}={value} in {:?}", store.filepath());
        }
        Command::Delete { key } => {
            store.delete(key)?;
            println!("delete {key} from {:?}", store.filepath());
        }
    }

    Ok(())
}
