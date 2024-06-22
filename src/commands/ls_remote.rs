use git2::{Direction, Repository};
use std::io;

/// List all references in a remote repository
pub(crate) fn ls_remote(url: &str) -> io::Result<()> {
    let temp_dir = tempfile::TempDir::new().unwrap();
    let repo = Repository::init_bare(temp_dir)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    let mut remote = repo
        .remote_anonymous(url)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    remote
        .connect_auth(Direction::Fetch, None, None)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    let list = remote
        .list()
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    for head in list {
        println!("{}\t{}", head.oid(), head.name());
    }

    Ok(())
}
