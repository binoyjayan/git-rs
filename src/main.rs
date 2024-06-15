use clap::Parser;
use std::io;

mod cli;
mod commands;

use cli::{Cli, Commands};

fn main() -> io::Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Some(Commands::Init) => {
            commands::init::init()?;
        }
        Some(Commands::CatFile {
            pretty_print,
            object_hash,
        }) => {
            commands::cat_file::cat_file(pretty_print, &object_hash)?;
        }
        Some(Commands::HashObject { write_object, file }) => {
            commands::hash_object::hash_object(write_object, &file)?;
        }
        None => {
            println!("Supported commands");
            println!("init: Initialize git repository");
            println!("cat-file: cat file with pretty print");
        }
    }
    Ok(())
}
