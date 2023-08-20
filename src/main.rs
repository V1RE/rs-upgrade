use clap::Parser;
use cli::Cli;

mod cli;
mod root;

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(cli::Command::List) => println!("List!"),
        None => root::run(),
    }
}
