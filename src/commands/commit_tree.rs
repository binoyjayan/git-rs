use std::io;
use std::io::Write;

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
        "author Quantum Coder <quantum@coder.com> {} {}",
        local.timestamp(),
        offset
    );
    let committer_str = format!(
        "committer Quantum Coder <quantum@coder.com> {} {}",
        local.timestamp(),
        offset
    );
    commit_obj.extend_from_slice(author_str.as_bytes());
    commit_obj.push(b'\n');
    commit_obj.extend_from_slice(committer_str.as_bytes());
    commit_obj.push(b'\n');
    commit_obj.push(b'\n');
    commit_obj.extend_from_slice(message.as_bytes());
    commit_obj.push(b'\n');

    let mut writer = ObjWriter::new()?;
    let header = format!("commit {}\0", commit_obj.len());
    writer.write_all(header.as_bytes())?;
    writer.write_all(&commit_obj)?;
    let hash = writer.finalize()?;
    Ok(hash)
}
