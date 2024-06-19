use std::io::Write;
use std::{fs, io, path};

use crate::objects::{hasher::Hasher, writers::ObjWriter};

/// Hash an object. Optionally write the object into the object database
pub(crate) fn hash_object<P: AsRef<path::Path>>(
    write: bool,
    raw_file_name: &P,
) -> io::Result<Vec<u8>> {
    let metadata = fs::metadata(raw_file_name)?;
    let raw_file = fs::File::open(raw_file_name)?;
    let mut reader = io::BufReader::new(raw_file);

    let file_size = metadata.len();
    let header = format!("blob {}\0", file_size);

    let object_hash = if write {
        let mut writer = ObjWriter::new()?;
        writer.write_all(header.as_bytes())?;
        io::copy(&mut reader, &mut writer)?;
        writer.finalize()?
    } else {
        let mut hasher = Hasher::new();
        hasher.write_all(header.as_bytes())?;
        io::copy(&mut reader, &mut hasher)?;
        hasher.finalize()
    };

    Ok(object_hash)
}
