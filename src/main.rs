use clap::Parser;
use std::io;

mod cli;
mod commands;
mod objects;
mod tree;

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
        Some(Commands::LsTree {
            name_only,
            object_hash,
        }) => {
            commands::ls_tree::ls_tree(name_only, &object_hash)?;
        }
        None => {
            println!("Supported commands");
            println!("init: Initialize git repository");
            println!("cat-file: cat file with pretty print");
            println!("hash-object: Create a hash of an object");
            println!("ls-tree: List the contents of a tree object");
        }
    }
    Ok(())
}
