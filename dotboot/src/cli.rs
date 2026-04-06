use clap::{Parser, ValueEnum};

#[derive(Parser, Debug)]
#[command(name = "dotboot")]
pub struct Cli {
    #[arg(short, long, default_value = ".boot.toml")]
    pub config: String,

    #[arg(short, long)]
    pub dry_run: bool,

    #[arg(short, long)]
    pub force: bool,

    pub command: Command,
}

#[derive(ValueEnum, Debug, Clone)]
pub enum Command {
    Install,
    Remove,
}
