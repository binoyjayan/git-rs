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

    /// Finalize the object writing process and create the object file
    /// with the hash of the object as the file name. Return the hash
    /// of the object. This function takes ownership of the writer so
    /// it can't be used after this function is called.    
    pub(crate) fn finalize(self) -> io::Result<Vec<u8>> {
        let tmp_path = self.file_path.clone();
        let hash = self.hasher.finalize().to_vec();
        let hash_str = hex::encode(&hash);
        // The first two characters of the object hash is the directory
        // that contains the object file, the rest is the file name.
        let obj_dir = &hash_str[..2];
        let obj_file = &hash_str[2..];
        let object_dir = format!(".git/objects/{}", obj_dir);
        let object_file = format!(".git/objects/{}/{}", obj_dir, obj_file);
        fs::create_dir_all(object_dir)?;
        fs::rename(tmp_path, object_file)?;
        Ok(hash)
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
