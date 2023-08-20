use clap::{Parser, Subcommand};

/// Update your system
///
/// A longer description that spans multiple lines and likely contains examples and usage of using your application.
#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    List,
}
