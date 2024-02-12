# Splitter

A very small utility for dividing large files into pieces.

## Why?

I was inspired to create this because a certain cloud storage service limits the size of file I
can upload without paying more, in spite of providing multiple petabytes of total space. Rather
than pay for the extra, I decided to game the system and make this tool.

Beyond cloud storage, there are undoubtedly other scenarios in which the sizes of individual
files are limited, and perhaps other reasons one may have for splitting a file.

## How?

Splitter opens the given `impl `[`Read`] and repeatedly reads the specified maximum number of bytes,
copying the result of each read into the next numbered file in the target directory.

Splitter does not perform any compression, nor does it have any awareness of the contents of
the file. It simply copies the data out, byte for byte. The input doesn't even need to be a
complete file.

WARNING: If any of the provided paths already exist, they will be overwritten. (TODO?)

## Example

```rust
use std::{
    fs::{self, File},
    path::Path,
};

const SOURCE: &str = "original";
const DIR: &str = "split";
const CHUNK_SIZE: u64 = 3;
const DATA: &[u8] = b"I love you";
const TARGET: &str = "reassembled";

fn main() {
    _ = fs::write(SOURCE, DATA);

    _ = splitter::split(&mut File::open(SOURCE).unwrap(), DIR, CHUNK_SIZE);

    let dir = AsRef::<Path>::as_ref(DIR);
    let mut chunks = DATA.chunks(CHUNK_SIZE as usize);
    assert_eq!(&fs::read(dir.join("0")).unwrap(), chunks.next().unwrap());
    assert_eq!(&fs::read(dir.join("1")).unwrap(), chunks.next().unwrap());

    _ = splitter::join(DIR, TARGET);

    assert_eq!(fs::read_to_string(SOURCE).unwrap(), fs::read_to_string(TARGET).unwrap());
}
```