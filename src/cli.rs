use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub(crate) struct Cli {
    #[command(subcommand)]
    pub(crate) command: Option<Commands>,
}

#[derive(clap::Subcommand)]
pub(crate) enum Commands {
    /// Initialize git repository
    Init,
    /// cat file with pretty print
    CatFile {
        #[clap(short, long)]
        /// Pretty print the object
        pretty_print: bool,
        /// The object hash of the file
        object_hash: String,
    },
    /// Create a hash of an object
    HashObject {
        #[clap(short, long)]
        write_object: bool,
        file: String,
    },
    /// List the contents of a tree object
    LsTree {
        /// Only display the name of the object
        #[clap(short, long)]
        name_only: bool,
        /// The object hash of the tree
        object_hash: String,
    },
    /// Write a tree object for a directory
    WriteTree {
        /// prefix for the directory
        #[clap(short, long)]
        prefix: Option<String>,
    },
}
