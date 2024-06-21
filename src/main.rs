use clap::Parser;
use std::io;

mod cli;
mod commands;
mod objects;

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
            let object_hash = commands::hash_object::hash_object(write_object, &file)?;
            println!("{}", hex::encode(object_hash));
        }
        Some(Commands::LsTree {
            name_only,
            object_hash,
        }) => {
            commands::ls_tree::ls_tree(name_only, &object_hash)?;
        }
        Some(Commands::WriteTree { prefix }) => {
            commands::write_tree::write_tree(prefix)?;
        }
        Some(Commands::CommitTree {
            object_hash,
            parent_commit,
            message,
        }) => {
            let commit_hash = commands::commit::commit_tree(&object_hash, parent_commit, &message)?;
            println!("{}", hex::encode(commit_hash));
        }
        Some(Commands::Commit { message }) => {
            commands::commit::commit(&message)?;
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
