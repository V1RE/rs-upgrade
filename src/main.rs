use clap::Parser;
use cli::Cli;
use cmd::{list, root};

mod cli;
mod cmd;
mod updater;

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(cli::Command::List) => list::run(),
        None => root::run(),
    }
}
