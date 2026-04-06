mod cli;
mod config;
mod fs;
mod symlink;

type Error = Box<dyn std::error::Error>;

use clap::Parser;
use cli::Cli;

fn main() -> Result<(), Error> {
    let cli = Cli::parse();
    symlink::run(cli)
}
