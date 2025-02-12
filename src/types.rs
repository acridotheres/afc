use std::fmt::Debug;

use dh::Readable;

#[derive(Debug, Clone)]
pub struct File {
    pub path: String,
    pub offset: u64,
    pub length: u64,
}

pub struct FileSource<'a, 'b>(pub File, pub &'a mut dyn Readable<'b>);
