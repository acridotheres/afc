use crate::FileSource;
use dh::{recommended::*, Writable};
use std::io::Result;

pub fn create<'a>(
    files: &mut [FileSource<'_, 'a>],
    target: &mut dyn Writable<'a>,
    buffer_size: u64,
) -> Result<()> {
    for FileSource(file, _) in files.iter_mut() {
        target.write_vu7(file.path.len() as u128)?;
        target.write_utf8(&file.path)?;
    }
    target.write_u8(0)?;

    for FileSource(file, source) in files {
        target.write_vu15le(file.length as u128)?;
        source.copy_at(file.offset, file.length, target, buffer_size)?;
    }

    Ok(())
}
