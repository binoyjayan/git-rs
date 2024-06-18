use std::io;
use std::io::{BufRead, Read};

use crate::objects::obj::{FileType, TreeMode};

pub(crate) struct TreeEntry {
    pub(crate) mode: TreeMode,
    pub(crate) name: String,
    pub(crate) sha: [u8; 20],
}

pub(crate) struct TreeReader<R> {
    reader: R,
}

impl<R> TreeReader<R>
where
    R: Read + BufRead,
{
    pub fn new(reader: R) -> Self {
        TreeReader { reader }
    }
}

/// Read the contents of a tree object
/// The format of a tree object is:
/// <mode> <name>\0<20-byte-sha>
/// where:
/// - mode is the file mode
/// - name is the name of the file
/// - sha is the sha of the object
/// The entries are separated by a null byte
impl<R> TreeReader<R>
where
    R: Read + BufRead,
{
    pub fn read(&mut self) -> io::Result<Vec<TreeEntry>> {
        let mut entries = Vec::new();

        loop {
            let mut mode = Vec::new();
            self.reader.read_until(b' ', &mut mode)?;
            if mode.is_empty() {
                break;
            }
            let mode_str = String::from_utf8(mode)
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.utf8_error()))?;

            let mode_int = mode_str
                .trim()
                .parse::<u32>()
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

            let mode = match mode_int {
                40000 => TreeMode::Directory,
                100644 => TreeMode::File(FileType::NonExecutable),
                100755 => TreeMode::File(FileType::Executable),
                160000 => TreeMode::Submodule,
                120000 => TreeMode::Symlink,
                other => TreeMode::Other(other),
            };

            let mut name = Vec::new();
            self.reader.read_until(b'\0', &mut name)?;
            name.pop(); // remove the last byte which is the null byte
            let name = String::from_utf8(name)
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.utf8_error()))?;

            let mut sha = [0; 20];
            self.reader.read_exact(&mut sha)?;

            entries.push(TreeEntry { mode, name, sha });
        }

        Ok(entries)
    }
}
