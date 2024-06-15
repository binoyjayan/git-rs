use flate2::read::ZlibDecoder;
use std::{fs, io};

/// cat file with pretty print and an object hash
pub(crate) fn cat_file(_pretty_print: bool, object_hash: &str) -> io::Result<()> {
    // The first two characters of the object hash is the directory
    // that contains the object file, the rest is the file name.
    let object_dir = &object_hash[..2];
    let object_file = &object_hash[2..];
    let object_path = format!(".git/objects/{}/{}", object_dir, object_file);
    let file = fs::File::open(object_path)?;
    let reader = io::BufReader::new(file);
    let mut decoder = ZlibDecoder::new(reader);
    io::copy(&mut decoder, &mut io::stdout())?;
    Ok(())
}
