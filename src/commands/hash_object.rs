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

        let (tmp_path, hash) = writer.finalize();
        let hash_str = hex::encode(&hash);

        // The first two characters of the object hash is the directory
        // that contains the object file, the rest is the file name.
        let obj_dir = &hash_str[..2];
        let obj_file = &hash_str[2..];
        let object_dir = format!(".git/objects/{}", obj_dir);
        let object_file = format!(".git/objects/{}/{}", obj_dir, obj_file);
        fs::create_dir_all(object_dir)?;
        fs::rename(tmp_path, object_file)?;
        hash
    } else {
        let mut hasher = Hasher::new();
        hasher.write_all(header.as_bytes())?;
        io::copy(&mut reader, &mut hasher)?;
        hasher.finalize()
    };

    Ok(object_hash)
}
