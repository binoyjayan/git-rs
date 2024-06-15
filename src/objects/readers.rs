use flate2::read::ZlibDecoder;
use std::fs;
use std::io::{self, BufRead, Read};
use std::path::PathBuf;

use crate::objects::obj::ObjectKind;

pub(crate) struct ObjReader {
    pub(crate) reader: io::BufReader<ZlibDecoder<fs::File>>,
    pub(crate) kind: ObjectKind,
}

impl ObjReader {
    pub fn new(object_hash: &str) -> io::Result<Self> {
        let object_dir = &object_hash[..2];
        let object_file = &object_hash[2..];
        let path = PathBuf::from(format!(".git/objects/{}/{}", object_dir, object_file));

        let file = fs::File::open(path)?;
        let decoder = ZlibDecoder::new(file);
        let mut reader = io::BufReader::new(decoder);

        // Content of the file is of format: blob <size>\0<content>
        // Read until the null byte to get the header
        let mut header = Vec::new();
        let mut byte = [0; 1];
        while reader.read_exact(&mut byte).is_ok() {
            if byte[0] == b'\0' {
                break;
            }
            header.push(byte[0]);
        }
        // Parse header of format: "blob <size>"
        let header = String::from_utf8(header).unwrap();
        let mut header = header.split_whitespace();
        // convert option to io::result when none
        let object_type = header
            .next()
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Invalid object_type"))?;

        let kind = match object_type {
            "blob" => ObjectKind::Blob,
            "commit" => ObjectKind::Commit,
            "tree" => ObjectKind::Tree,
            "tag" => ObjectKind::Tag,
            other => ObjectKind::Other(other.to_string()),
        };

        Ok(ObjReader { reader, kind })
    }
}

impl Read for ObjReader {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.reader.read(buf)
    }
}

impl BufRead for ObjReader {
    fn fill_buf(&mut self) -> io::Result<&[u8]> {
        self.reader.fill_buf()
    }

    fn consume(&mut self, amt: usize) {
        self.reader.consume(amt)
    }
}
