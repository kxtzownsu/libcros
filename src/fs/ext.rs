use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

use super::{EXT_MAGIC, EXT_MAGIC_OFFSET, EXT_SUPERBLOCK_OFFSET};

pub fn verify_ext(f: &mut File, base: u64) -> bool {
  let mut buf = [0u8; 2];

  if f.seek(SeekFrom::Start(base + EXT_SUPERBLOCK_OFFSET + EXT_MAGIC_OFFSET)).is_err() {
    return false;
  }

  if f.read_exact(&mut buf).is_err() {
    return false;
  }

  buf == EXT_MAGIC
}
