use clap::{Parser, ValueEnum};

mod http;

use http::send_request_http;

#[derive(Parser)]
struct Args {
    #[arg(value_enum)]
    command: Command,
}

#[derive(Clone, ValueEnum)]
enum Command {
    HTTP, // Layer 7
}

fn main() {
    let args = Args::parse();

    match args.command {
        Command::HTTP => send_request_http().expect("HTTP request failed"),
    }
}
