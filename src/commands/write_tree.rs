use std::io::Write;
use std::os::unix::fs::PermissionsExt;
use std::{fs, io, path};

use crate::{
    commands::hash_object::hash_object,
    objects::{
        hasher::Hasher,
        obj::{FileType, TreeMode},
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
        None => fs::canonicalize(".")?,
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
        // let entry = entry?;
        // let path = entry.path();
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
        } else if meta.is_symlink() {
            TreeMode::Symlink
        } else {
            let file_type = meta.file_type();
            let mode = if file_type.is_file() {
                if meta.permissions().mode() & 0o111 != 0 {
                    FileType::Executable
                } else {
                    FileType::NonExecutable
                }
            } else {
                FileType::NonExecutable
            };
            TreeMode::File(mode)
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
            false => hash_object(false, &path)?,
        };
        let mode_str = format!("{:o}", mode);
        println!("{} {} {}", mode_str, hex::encode(&hash), file_name);
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
        let mut hasher = Hasher::new();
        hasher.write_all(header.as_bytes())?;
        hasher.write_all(&tree_obj)?;
        let hash = hasher.finalize();
        // let mut writer = ObjWriter::new()?;
        // writer.write_all(header.as_bytes())?;
        // writer.write_all(&tree_obj)?;
        // let (tmp_path, hash) = writer.finalize();
        // let hash_str = hex::encode(&hash);
        // let obj_dir = &hash_str[..2];
        // let obj_file = &hash_str[2..];
        // let object_dir = format!(".git/objects/{}", obj_dir);
        // let object_file = format!(".git/objects/{}/{}", obj_dir, obj_file);
        // fs::create_dir_all(object_dir)?;
        // fs::rename(tmp_path, object_file)?;
        Ok(Some(hash))
    }
}
