use std::io;
use std::io::{BufRead, Read};

pub(crate) struct TreeEntry {
    pub(crate) mode: String,
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
            let mode = String::from_utf8(mode).unwrap();

            let mut name = Vec::new();
            self.reader.read_until(b'\0', &mut name)?;
            let name = String::from_utf8(name).unwrap();

            let mut sha = [0; 20];
            self.reader.read_exact(&mut sha)?;

            entries.push(TreeEntry { mode, name, sha });
        }

        Ok(entries)
    }
}
