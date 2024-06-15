use std::io;

use crate::objects::{obj::ObjectKind, readers::ObjReader};

/// cat file with pretty print and an object hash
pub(crate) fn cat_file(_pretty_print: bool, object_hash: &str) -> io::Result<()> {
    let mut reader = ObjReader::new(object_hash)?;

    match reader.kind {
        ObjectKind::Blob | ObjectKind::Commit => {
            io::copy(&mut reader, &mut io::stdout())?;
        }
        ObjectKind::Tree => {
            // let mut entry = Vec::new();
            // while reader.decoder.read_until(b'\0', &mut entry)? != 0 {
            //     let entry = String::from_utf8(entry).unwrap();
            //     let mut entry = entry.split_whitespace();
            //     let mode = entry.next().unwrap();
            //     let object_hash = entry.next().unwrap();
            //     let object_type = entry.next().unwrap();
            //     let object_name = entry.collect::<Vec<&str>>().join(" ");
            //     println!("{} {} {} {}", mode, object_type, object_hash, object_name);
            //     entry.clear();
            // }
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Object type tree is not supported",
            ));
        }
        ObjectKind::Tag => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Object type tag is not supported",
            ));
        }
        ObjectKind::Other(s) => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Invalid object_type {}", s),
            ));
        }
    }
    Ok(())
}
