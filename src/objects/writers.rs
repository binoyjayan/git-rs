use flate2::{write::ZlibEncoder, Compression};
use sha1::{Digest, Sha1};
use std::fs;
use std::io::{self, BufWriter, Write};
use std::path::PathBuf;

pub(crate) struct ObjWriter {
    file_path: PathBuf,
    writer: BufWriter<ZlibEncoder<fs::File>>,
    hasher: Sha1,
}

impl ObjWriter {
    pub fn new() -> io::Result<Self> {
        let uuid = uuid::Uuid::now_v7();
        let file_name = format!(".git/objects/tmp_{}", uuid);
        let file_path = PathBuf::from(&file_name);
        let tmp_file = std::fs::File::create(&file_name)?;
        let encoder = ZlibEncoder::new(tmp_file, Compression::default());
        let writer = BufWriter::new(encoder);

        Ok(Self {
            file_path,
            writer,
            hasher: Sha1::new(),
        })
    }

    pub(crate) fn finalize(self) -> (PathBuf, Vec<u8>) {
        let path = self.file_path.clone();
        let hash = self.hasher.finalize().to_vec();
        (path, hash)
    }
}

impl Write for ObjWriter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let len = self.writer.write(buf)?;
        self.hasher.update(&buf[..len]);
        Ok(len)
    }

    fn write_all(&mut self, data: &[u8]) -> std::io::Result<()> {
        self.writer.write_all(data)?;
        self.hasher.update(data);
        Ok(())
    }

    fn flush(&mut self) -> io::Result<()> {
        self.writer.flush()
    }
}

// pub(crate) struct HashWriter<W> {
//     pub(crate) writer: W,
//     pub(crate) hasher: sha1::Sha1,
// }

// impl<W> Write for HashWriter<W>
// where
//     W: Write,
// {
//     fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
//         let n = self.writer.write(buf)?;
//         self.hasher.update(&buf[..n]);
//         Ok(n)
//     }

//     fn flush(&mut self) -> std::io::Result<()> {
//         self.writer.flush()
//     }
// }
