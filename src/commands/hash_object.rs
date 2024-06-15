use flate2::{write::ZlibEncoder, Compression};
use sha1::Digest;
use std::io::Write;
use std::{fs, io};

use crate::objects::writers::HashWriter;

/// Hash an object. Optionally write the object into the object database
pub(crate) fn hash_object(write: bool, raw_file_name: &str) -> io::Result<()> {
    let metadata = fs::metadata(raw_file_name)?;
    let raw_file = fs::File::open(raw_file_name)?;
    let mut reader = io::BufReader::new(raw_file);
    // Get the file size
    let file_size = metadata.len();
    let header = format!("blob {}\0", file_size);
    // Create a temporary object file to write the object to
    let tmp_file = tempfile::NamedTempFile::new_in(".git/objects")?;

    let object_hash = if write {
        // tmp_file is reopened so that it does not get dropped as it goes out of scope
        let writer = io::BufWriter::new(tmp_file.reopen()?);
        let encoder = ZlibEncoder::new(writer, Compression::default());
        let mut hash_writer = HashWriter {
            writer: encoder,
            hasher: sha1::Sha1::new(),
        };
        // Write header of format: 'blob <size>\0' to the object database

        hash_writer.write_all(header.as_bytes())?;
        io::copy(&mut reader, &mut hash_writer)?;
        let _ = hash_writer.writer.finish();
        let object_hash = hash_writer.hasher.finalize();
        let object_hash = hex::encode(object_hash);

        // The first two characters of the object hash is the directory
        // that contains the object file, the rest is the file name.
        let obj_dir = &object_hash[..2];
        let obj_file = &object_hash[2..];
        let object_dir = format!(".git/objects/{}", obj_dir);
        let object_file = format!(".git/objects/{}/{}", obj_dir, obj_file);
        fs::create_dir_all(object_dir)?;
        fs::rename(tmp_file.path(), object_file)?;
        object_hash
    } else {
        let mut hash_writer = HashWriter {
            writer: io::sink(),
            hasher: sha1::Sha1::new(),
        };
        hash_writer.write_all(header.as_bytes())?;
        io::copy(&mut reader, &mut hash_writer)?;
        let object_hash = hash_writer.hasher.finalize();
        hex::encode(object_hash)
    };

    println!("{}", object_hash);

    Ok(())
}
