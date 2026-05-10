pub mod ext;
pub mod fat;

use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

pub mod filesystems {
  pub const EXT2: &str = "ext2";
  pub const EXT3: &str = "ext3";
  pub const EXT4: &str = "ext4";
  pub const FAT16: &str = "fat16";
  pub const FAT32: &str = "fat32";
}

pub const EXT_SUPERBLOCK_OFFSET: u64 = 1024;
pub const EXT_MAGIC_OFFSET: u64 = 0x38;
pub const EXT_MAGIC: [u8; 2] = [0x53, 0xef];

pub const FAT_LABEL_LEN: usize = 8;
pub const FAT16_LABEL_OFFSET: u64 = 54;
pub const FAT32_LABEL_OFFSET: u64 = 82;
pub const FAT16_LABEL: &str = "FAT16";
pub const FAT32_LABEL: &str = "FAT32";

pub fn verify_filesystem(fs: &str, f: &mut File, base: u64) -> bool {
  match fs {
    filesystems::EXT2 | filesystems::EXT3 | filesystems::EXT4 => ext::verify_ext(f, base),
    filesystems::FAT16 => fat::verify_fat16(f, base),
    filesystems::FAT32 => fat::verify_fat32(f, base),
    _ => false,
  }
}

pub fn get_filesystem(f: &mut File, base: u64) -> Option<&'static str> {
  let mut ext_magic = [0u8; 2];
  if f.seek(SeekFrom::Start(base + EXT_SUPERBLOCK_OFFSET + EXT_MAGIC_OFFSET)).is_ok()
    && f.read_exact(&mut ext_magic).is_ok()
  {
    if ext_magic == EXT_MAGIC {
      return Some("ext");
    }
  }

  let mut fat16 = [0u8; FAT_LABEL_LEN];
  if f.seek(SeekFrom::Start(base + FAT16_LABEL_OFFSET)).is_ok()
    && f.read_exact(&mut fat16).is_ok()
  {
    if core::str::from_utf8(&fat16)
      .map(|s| s.contains(FAT16_LABEL))
      .unwrap_or(false)
    {
      return Some(filesystems::FAT16);
    }
  }

  let mut fat32 = [0u8; FAT_LABEL_LEN];
  if f.seek(SeekFrom::Start(base + FAT32_LABEL_OFFSET)).is_ok()
    && f.read_exact(&mut fat32).is_ok()
  {
    if core::str::from_utf8(&fat32)
      .map(|s| s.contains(FAT32_LABEL))
      .unwrap_or(false)
    {
      return Some(filesystems::FAT32);
    }
  }

  None
}
