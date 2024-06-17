use std::io;

use crate::objects::{obj::ObjectKind, readers::ObjReader, tree::TreeReader};

/// cat file with pretty print and an object hash
pub(crate) fn cat_file(_pretty_print: bool, object_hash: &str) -> io::Result<()> {
    let mut reader = ObjReader::new(object_hash)?;

    match reader.kind {
        ObjectKind::Blob | ObjectKind::Commit => {
            io::copy(&mut reader, &mut io::stdout())?;
        }
        ObjectKind::Tree => {
            let mut tree_reader = TreeReader::new(&mut reader);
            let entries = tree_reader.read()?;
            for entry in entries {
                println!(
                    "{} {} {}    {}",
                    entry.mode,
                    entry.mode.as_str(),
                    hex::encode(entry.sha),
                    entry.name
                );
            }
        }
        ObjectKind::Tag => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Object type tag is not supported",
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
