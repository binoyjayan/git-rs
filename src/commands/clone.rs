use git2::{build::RepoBuilder, FetchOptions, RemoteCallbacks};
use std::{io, path};

/// Clone a remote repository to a local directory
pub(crate) fn clone_repo(url: &str, local_path: Option<&str>) -> io::Result<()> {
    let repo_name = url.rsplit('/').next().unwrap_or("repo");
    let path = local_path.unwrap_or(repo_name);

    let mut cb = RemoteCallbacks::new();
    cb.transfer_progress(|stats| {
        println!(
            "Received {}/{} objects in {} bytes (used {} local objects)",
            stats.indexed_objects(),
            stats.total_objects(),
            stats.received_bytes(),
            stats.local_objects()
        );
        true
    });

    let mut fo = FetchOptions::new();
    fo.remote_callbacks(cb);

    let result = RepoBuilder::new()
        .fetch_options(fo)
        .clone(url, path::Path::new(path));

    match result {
        Ok(_) => println!("\nDone."),
        Err(e) => return Err(io::Error::new(io::ErrorKind::Other, e)),
    };

    Ok(())
}
