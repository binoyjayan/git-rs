use sha1::Digest;
use std::io::Write;

pub(crate) struct HashWriter<W> {
    pub(crate) writer: W,
    pub(crate) hasher: sha1::Sha1,
}

impl<W> Write for HashWriter<W>
where
    W: Write,
{
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let n = self.writer.write(buf)?;
        self.hasher.update(&buf[..n]);
        Ok(n)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.writer.flush()
    }
}
