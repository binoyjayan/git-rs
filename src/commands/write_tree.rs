use std::io::Write;
use std::os::unix::fs::PermissionsExt;
use std::{fs, io, path};

use crate::{
    commands::hash_object::hash_object,
    objects::{
        obj::{BlobType, TreeMode},
        writers::ObjWriter,
    },
};

/// Write a tree object from a directory or subdirectory
pub(crate) fn write_tree(prefix: Option<String>) -> io::Result<()> {
    let dir = match prefix {
        Some(p) => {
            // Convert p to pathbuf
            let path = fs::canonicalize(p)?;
            if !path.is_dir() {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "Not a directory",
                ));
            }
            path
        }
        None => path::PathBuf::from("."),
    };

    if let Some(hash) = write_subtree(&dir)? {
        println!("{}", hex::encode(hash));
    }

    Ok(())
}

/// Write a subtree object recursively and return the hash of the object
pub(crate) fn write_subtree(dir: &path::PathBuf) -> io::Result<Option<Vec<u8>>> {
    // Iterate over the directory
    let mut entries: Vec<_> = fs::read_dir(dir)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<_, io::Error>>()?;
    entries.sort();

    let mut tree_obj = Vec::new();
    for path in entries {
        let meta = path.metadata()?;
        // Ignore .git directory
        if path.ends_with(".git") {
            continue;
        }
        let file_name = match path.file_name().and_then(|n| n.to_str()) {
            Some(name) => name.to_owned(),
            None => continue,
        };

        let mode: u32 = if meta.is_dir() {
            TreeMode::Directory
        } else if meta.is_symlink() {
            TreeMode::Symlink
        } else if meta.is_file() {
            let file_type = meta.file_type();
            let mode = if file_type.is_file() {
                if meta.permissions().mode() & 0o111 != 0 {
                    BlobType::Executable
                } else {
                    BlobType::NonExecutable
                }
            } else {
                BlobType::NonExecutable
            };
            TreeMode::Blob(mode)
        } else {
            continue;
        }
        .into();
        let hash = match meta.is_dir() {
            true => {
                // if the path is a directory, call write_subtree recursively
                if let Some(hash) = write_subtree(&path)? {
                    hash
                } else {
                    continue;
                }
            }
            false => hash_object(true, &path)?,
        };
        let mode_str = format!("{:o}", mode);
        // println!("{} {} {}", mode_str, hex::encode(&hash), file_name);
        tree_obj.extend_from_slice(mode_str.as_bytes());
        tree_obj.push(b' ');
        tree_obj.extend_from_slice(file_name.as_bytes());
        tree_obj.push(b'\0');
        tree_obj.extend_from_slice(&hash);
    }
    if tree_obj.is_empty() {
        Ok(None)
    } else {
        let header = format!("tree {}\0", tree_obj.len());
        let mut writer = ObjWriter::new()?;
        writer.write_all(header.as_bytes())?;
        writer.write_all(&tree_obj)?;
        let hash = writer.finalize()?;
        Ok(Some(hash))
    }
}
