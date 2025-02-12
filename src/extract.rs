use crate::FileSource;
use dh::{recommended::*, Writable};
use std::io::Result;

pub fn extract<'a>(
    source: &mut FileSource<'_, 'a>,
    target: &mut dyn Writable<'a>,
    buffer_size: u64,
) -> Result<()> {
    source
        .1
        .copy_at(source.0.offset, source.0.length, target, buffer_size)
}
