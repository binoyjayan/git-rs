use std::fmt;

#[derive(Debug, Clone)]
pub(crate) enum ObjectKind {
    Blob,
    Commit,
    Tree,
    Tag,
    Other(String),
}

#[derive(Debug, Clone)]

pub(crate) enum BlobType {
    Executable,
    NonExecutable,
}

#[derive(Debug, Clone)]
pub(crate) enum TreeMode {
    Directory,
    Blob(BlobType),
    Submodule,
    Symlink,
    Other(u32),
}

impl fmt::Display for ObjectKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ObjectKind::Blob => write!(f, "blob"),
            ObjectKind::Commit => write!(f, "commit"),
            ObjectKind::Tree => write!(f, "tree"),
            ObjectKind::Tag => write!(f, "tag"),
            ObjectKind::Other(ref s) => write!(f, "{}", s),
        }
    }
}

impl fmt::Display for BlobType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BlobType::Executable => write!(f, "100755"),
            BlobType::NonExecutable => write!(f, "100644"),
        }
    }
}

impl fmt::Display for TreeMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TreeMode::Directory => write!(f, "040000"),
            TreeMode::Blob(ref ft) => write!(f, "{}", ft),
            TreeMode::Submodule => write!(f, "160000"),
            TreeMode::Symlink => write!(f, "120000"),
            TreeMode::Other(ref n) => write!(f, "{}", n),
        }
    }
}

impl TreeMode {
    /// Return the string representation of the tree mode
    pub fn as_str(&self) -> &'static str {
        match self {
            TreeMode::Directory => "tree",
            TreeMode::Blob(ft) => match ft {
                BlobType::Executable => "blob",
                BlobType::NonExecutable => "blob",
            },
            TreeMode::Submodule => "commit",
            TreeMode::Symlink => "blob",
            TreeMode::Other(_) => "unknown",
        }
    }
}

// Convert to numeric representation
impl From<TreeMode> for u32 {
    fn from(mode: TreeMode) -> u32 {
        match mode {
            TreeMode::Directory => 0o040000,
            TreeMode::Blob(ft) => match ft {
                BlobType::Executable => 0o100755,
                BlobType::NonExecutable => 0o100644,
            },
            TreeMode::Submodule => 0o160000,
            TreeMode::Symlink => 0o120000,
            TreeMode::Other(n) => n,
        }
    }
}
