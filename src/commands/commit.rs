use std::io::Write;
use std::{fs, io, path};

use crate::commands::write_tree::write_subtree;
use crate::objects::writers::ObjWriter;

/// Commit a tree object
pub fn commit_tree(
    object_hash: &str,
    parent_commit: Option<String>,
    message: &str,
) -> io::Result<Vec<u8>> {
    let mut commit_obj = Vec::new();
    commit_obj.extend_from_slice(b"tree ");
    commit_obj.extend_from_slice(object_hash.as_bytes());
    commit_obj.push(b'\n');
    if let Some(parent_commit) = parent_commit {
        commit_obj.extend_from_slice(b"parent ");
        commit_obj.extend_from_slice(parent_commit.as_bytes());
        commit_obj.push(b'\n');
    }
    // Get current time in seconds since epoch and timezone offset
    let local: chrono::DateTime<chrono::Local> = chrono::Local::now();
    let offset = local.offset().local_minus_utc();
    let hours = offset / 3600;
    let minutes = (offset.abs() % 3600) / 60;
    let offset = format!("{:+03}{:02}", hours, minutes);
    let author_str = format!(
        "author Quantum Coder <coder@quantum.com> {} {}",
        local.timestamp(),
        offset
    );
    let committer_str = format!(
        "committer Quantum Coder <coder@quantum.com> {} {}",
        local.timestamp(),
        offset
    );
    commit_obj.extend_from_slice(author_str.as_bytes());
    commit_obj.push(b'\n');
    commit_obj.extend_from_slice(committer_str.as_bytes());
    commit_obj.extend_from_slice(b"\n\n");
    commit_obj.extend_from_slice(message.as_bytes());
    commit_obj.push(b'\n');

    let mut writer = ObjWriter::new()?;
    let header = format!("commit {}\0", commit_obj.len());
    writer.write_all(header.as_bytes())?;
    writer.write_all(&commit_obj)?;
    let hash = writer.finalize()?;
    Ok(hash)
}

/// Commit a tree object
pub fn commit(message: &str) -> io::Result<()> {
    let tree_hash = write_subtree(&path::PathBuf::from("."))?;
    if let Some(tree_hash) = tree_hash {
        let tree_hash_str = hex::encode(tree_hash);
        // read refs_head: e.g. "ref: refs/heads/main"
        let refs_head = std::fs::read_to_string(".git/HEAD")?;
        if let Some(refs_head) = refs_head.strip_prefix("ref: ") {
            // Get the path to the refs file: ".git/refs/heads/master"
            let refs_branch = format!(".git/{}", refs_head.trim());
            let curr_commit = match std::fs::metadata(&refs_branch) {
                Ok(_) => {
                    let parent_commit = std::fs::read_to_string(&refs_branch)?.trim().to_string();
                    let commit_hash = commit_tree(&tree_hash_str, Some(parent_commit), message)?;
                    hex::encode(commit_hash)
                }
                Err(_) => {
                    // If the file does not exist, it is the first commit
                    let commit_hash = commit_tree(&tree_hash_str, None, message)?;
                    hex::encode(commit_hash)
                }
            };
            fs::create_dir_all(".git/refs/heads")?;
            let mut file = std::fs::File::create(&refs_branch)?;
            writeln!(file, "{}", curr_commit)?;
        } else {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid HEAD"));
        }
    } else {
        println!("No changes to commit");
    }
    Ok(())
}
