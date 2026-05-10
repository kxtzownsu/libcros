use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

use super::{FAT16_LABEL, FAT16_LABEL_OFFSET, FAT32_LABEL, FAT32_LABEL_OFFSET, FAT_LABEL_LEN};

fn read_label(f: &mut File, off: u64) -> Option<[u8; FAT_LABEL_LEN]> {
  let mut buf = [0u8; FAT_LABEL_LEN];

  f.seek(SeekFrom::Start(off)).ok()?;
  f.read_exact(&mut buf).ok()?;

  Some(buf)
}

pub fn verify_fat16(f: &mut File, base: u64) -> bool {
  match read_label(f, base + FAT16_LABEL_OFFSET) {
    Some(b) => core::str::from_utf8(&b).map(|s| s.contains(FAT16_LABEL)).unwrap_or(false),
    None => false,
  }
}

pub fn verify_fat32(f: &mut File, base: u64) -> bool {
  match read_label(f, base + FAT32_LABEL_OFFSET) {
    Some(b) => core::str::from_utf8(&b).map(|s| s.contains(FAT32_LABEL)).unwrap_or(false),
    None => false,
  }
}
