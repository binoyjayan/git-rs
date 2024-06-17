use std::io;

use crate::objects::{obj::ObjectKind, readers::ObjReader, tree::TreeReader};

/// List the contents of a tree object
pub(crate) fn ls_tree(name_only: bool, tree_hash: &str) -> io::Result<()> {
    let mut reader = ObjReader::new(tree_hash)?;
    match reader.kind {
        ObjectKind::Tree => {
            let mut tree_reader = TreeReader::new(&mut reader);
            let entries = tree_reader.read()?;
            for entry in entries {
                if name_only {
                    println!("{}", entry.name);
                } else {
                    println!("{} {} {}", entry.mode, hex::encode(entry.sha), entry.name);
                }
            }
        }
        ObjectKind::Blob | ObjectKind::Commit | ObjectKind::Tag => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "ls-tree only works with tree objects",
            ));
        }
        ObjectKind::Other(s) => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Invalid object_type {}", s),
            ));
        }
    }
    Ok(())
}
