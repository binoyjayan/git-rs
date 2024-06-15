pub(crate) enum ObjectKind {
    Blob,
    Commit,
    Tree,
    Tag,
    Other(String),
}
