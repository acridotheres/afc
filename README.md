# Asset File Container format

This format is optimized for size and has no magic bytes. The format can only be detected by it's file extension (.afc). It's fully intended to be combined with a compressor, resulting in file extensions like `.afc.gz`, `.afc.xz` or `.afc.zstd`.

Also, this format uses [7](https://docs.rs/dh/0.8.1/dh/trait.Readable.html#method.read_vu7) and [15](https://docs.rs/dh/0.8.1/dh/trait.Readable.html#method.read_vu15le) bit variable length integers allowing for theoretically infinite file size.

## Structure

File contents can be divided into two blocks:
1. File path list
2. File contents

### Path list

The first block is a list of strings in this structure:

- VU7: Length of the file path. If 0, this is the last entry and not a file, file content block will start after this byte.
- UTF-8: File path

### Content block

The second block is also a list but has the same item length as the path list. Every entry is in order with the part list so everything can be mapped 1:1.

An item has this structure:

- VU15LE: file content length
- U8[]: file content
