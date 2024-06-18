use sha1::Digest;
use std::io::Write;

pub(crate) struct Hasher {
    pub(crate) hasher: sha1::Sha1,
}

impl Hasher {
    pub fn new() -> Self {
        Self {
            hasher: sha1::Sha1::new(),
        }
    }
    pub fn finalize(self) -> Vec<u8> {
        self.hasher.finalize().to_vec()
    }
}

impl Write for Hasher {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.hasher.update(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
