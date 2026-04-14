use crate::structs::{read_struct, write_struct, GPTHeader, GPTPartitionEntry};

const EMPTY_GUID: [u8; 16] = [0u8; 16];

pub fn gpt_guid_to_uuid(raw: &[u8; 16]) -> uuid::Uuid {
  let mut bytes = *raw;
  bytes[0..4].reverse();
  bytes[4..6].reverse();
  bytes[6..8].reverse();
  uuid::Uuid::from_bytes(bytes)
}

pub fn uuid_to_gpt_guid(uuid: &uuid::Uuid) -> [u8; 16] {
  let mut bytes = *uuid.as_bytes();
  bytes[0..4].reverse();
  bytes[4..6].reverse();
  bytes[6..8].reverse();
  bytes
}

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

  pub fn partitions_by_type(&self, type_uuid: &uuid::Uuid) -> Vec<(u32, &GPTPartitionEntry)> {
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

  pub fn has_partition_type(&self, type_uuid: &uuid::Uuid) -> bool {
    let target = uuid_to_gpt_guid(type_uuid);
    self
      .entries
      .iter()
      .any(|entry| Self::entry_type_guid(entry) == target)
  }

  pub fn partition_with_type(&self, id: u32, type_uuid: &uuid::Uuid) -> Option<&GPTPartitionEntry> {
    let target = uuid_to_gpt_guid(type_uuid);
    let entry = self.partition(id)?;
    if Self::entry_type_guid(entry) != target {
      return None;
    }
    Some(entry)
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
