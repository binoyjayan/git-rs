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

pub(crate) enum FileType {
    Executable,
    NonExecutable,
}

#[derive(Debug, Clone)]
pub(crate) enum TreeMode {
    Directory,
    File(FileType),
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

impl fmt::Display for FileType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FileType::Executable => write!(f, "100755"),
            FileType::NonExecutable => write!(f, "100644"),
        }
    }
}

impl fmt::Display for TreeMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TreeMode::Directory => write!(f, "040000"),
            TreeMode::File(ref ft) => write!(f, "{}", ft),
            TreeMode::Submodule => write!(f, "160000"),
            TreeMode::Symlink => write!(f, "120000"),
            TreeMode::Other(ref n) => write!(f, "{}", n),
        }
    }
}

impl TreeMode {
    pub fn as_str(&self) -> &'static str {
        match self {
            TreeMode::Directory => "tree",
            TreeMode::File(ft) => match ft {
                FileType::Executable => "blob",
                FileType::NonExecutable => "blob",
            },
            TreeMode::Submodule => "commit",
            TreeMode::Symlink => "blob",
            TreeMode::Other(_) => "unknown",
        }
    }
}
