use dh::{recommended::*, Readable};
use std::io::Result;

use crate::File;

pub fn list(source: &mut dyn Readable) -> Result<Vec<File>> {
    let mut files = vec![];

    loop {
        let path_len = source.read_vu7()?;
        if path_len == 0 {
            break;
        }

        let path = source.read_utf8(path_len as u64)?;

        files.push(File {
            path,
            offset: 0,
            length: 0,
        });
    }

    for file in files.iter_mut() {
        file.length = source.read_vu15le()? as u64;
        file.offset = source.pos()?;
        source.jump(file.length as i64)?;
    }

    Ok(files)
}
