use flate2::read::ZlibDecoder;
use std::io::Read;
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
    // Content of the file is of format: blob <size>\0<content>
    // Read until the null byte to get the header
    let mut header = Vec::new();
    let mut byte = [0; 1];
    while decoder.read_exact(&mut byte).is_ok() {
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
    match object_type {
        "blob" | "commit" => {
            io::copy(&mut decoder, &mut io::stdout())?;
        }
        _ => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid object_type",
            ));
        }
    }

    Ok(())
}
