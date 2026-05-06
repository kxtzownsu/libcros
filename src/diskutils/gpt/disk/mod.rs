use std::io::{self, Read, Seek, SeekFrom};

use crate::structs::{read_struct, write_struct, GPTHeader, GPTPartitionEntry};
use super::{Uuid, gpt_guid_to_uuid, uuid_to_gpt_guid};

const EMPTY_GUID: [u8; 16] = [0u8; 16];
const GPT_HEADER_LBA: u64 = 1; /* https://en.wikipedia.org/wiki/GUID_Partition_Table#Partition_table_header_(LBA_1) */
const SECTOR_SIZE: u64 = 512; /* This is the default sector size. Maybe we shouldn't be hardcoding it? */

pub struct GptDisk {
  pub header: GPTHeader,
  pub entries: Vec<GPTPartitionEntry>,
}

impl GptDisk {
  fn partition_index(id: u32) -> Option<usize> {
    if id == 0 {
      return None;
    }
    Some(id as usize - 1)
  }

  fn entry_type_guid(entry: &GPTPartitionEntry) -> [u8; 16] {
    let field_ptr = core::ptr::addr_of!(entry.partition_type_guid) as *const u8;
    let field_bytes =
      unsafe { core::slice::from_raw_parts(field_ptr, core::mem::size_of::<[u8; 16]>()) };
    read_struct(field_bytes)
  }

  fn is_used_entry(entry: &GPTPartitionEntry) -> bool {
    Self::entry_type_guid(entry) != EMPTY_GUID
  }

  pub fn partition(&self, id: u32) -> Option<&GPTPartitionEntry> {
    let index = Self::partition_index(id)?;
    let entry = self.entries.get(index)?;
    if !Self::is_used_entry(entry) {
      return None;
    }
    Some(entry)
  }

  pub fn partition_mut(&mut self, id: u32) -> Option<&mut GPTPartitionEntry> {
    let index = Self::partition_index(id)?;
    self.entries.get_mut(index)
  }

  pub fn partitions_by_type(&self, type_uuid: &Uuid) -> Vec<(u32, &GPTPartitionEntry)> {
    let target = uuid_to_gpt_guid(type_uuid);
    self
      .entries
      .iter()
      .enumerate()
      .filter_map(|(i, entry)| {
        if Self::entry_type_guid(entry) != target {
          return None;
        }
        Some((i as u32 + 1, entry))
      })
      .collect()
  }

  pub fn has_partition_type(&self, type_uuid: &Uuid) -> bool {
    let target = uuid_to_gpt_guid(type_uuid);
    self
      .entries
      .iter()
      .any(|entry| Self::entry_type_guid(entry) == target)
  }

  pub fn partition_with_type(&self, id: u32, type_uuid: &Uuid) -> Option<&GPTPartitionEntry> {
    let target = uuid_to_gpt_guid(type_uuid);
    let entry = self.partition(id)?;
    if Self::entry_type_guid(entry) != target {
      return None;
    }
    Some(entry)
  }

  pub fn get_partition_type(&self, id: u32) -> Option<Uuid> {
    let entry = self.partition(id)?;
    Some(gpt_guid_to_uuid(&Self::entry_type_guid(entry)))
  }
}

pub fn serialize_entries(disk: &GptDisk) -> Vec<u8> {
  let entry_size = disk.header.size_partition_entry as usize;
  if entry_size == 0 {
    return Vec::new();
  }

  let mut raw = vec![0u8; disk.entries.len() * entry_size];
  for (i, entry) in disk.entries.iter().enumerate() {
    write_struct(entry, &mut raw[i * entry_size..]);
  }

  raw
}

pub fn read_entries<R: Read + Seek>(
  disk: &mut R,
  header: &GPTHeader,
) -> io::Result<Vec<GPTPartitionEntry>> {
  disk.seek(SeekFrom::Start(header.partition_entry_lba * SECTOR_SIZE))?;

  let entry_size = header.size_partition_entry as usize;
  let count = header.num_partition_entries as usize;

  if entry_size == 0 || count == 0 {
    return Ok(Vec::new());
  }

  let struct_size = core::mem::size_of::<GPTPartitionEntry>();
  let read_size = entry_size.max(struct_size);
  let mut entries = Vec::with_capacity(count);

  for _ in 0..count {
    let mut buf = vec![0u8; read_size];
    disk.read_exact(&mut buf[..entry_size])?;
    entries.push(read_struct(&buf));
  }

  Ok(entries)
}

pub fn read_header<R: Read + Seek>(disk: &mut R) -> io::Result<GPTHeader> {
  disk.seek(SeekFrom::Start(GPT_HEADER_LBA * SECTOR_SIZE))?;

  let header_size = core::mem::size_of::<GPTHeader>();
  let mut header_bytes = vec![0u8; header_size];
  disk.read_exact(&mut header_bytes)?;

  Ok(read_struct(&header_bytes))
}
