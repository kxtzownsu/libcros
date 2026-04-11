#[cfg(feature = "tlcl")]
#[derive(Debug)]
pub struct TPM2B {
  pub size: u16,
  pub buffer: *const u8,
}

#[cfg(feature = "tlcl")]
#[derive(Debug)]
pub struct Tpm2SessionHeader {
  pub session_handle: u32,
  pub nonce_size: u16,
  pub nonce: u8,
  pub session_attrs: u8,
  pub auth_size: u16,
  pub auth: u8,
}

#[cfg(feature = "tlcl")]
#[derive(Debug)]
pub struct Tpm2TpmHeader {
  pub tpm_tag: u16,
  pub tpm_size: u32,
  pub tpm_code: u32,
}

#[cfg(feature = "diskutils")]
#[repr(C, packed)]
#[derive(Debug, Clone)]
pub struct GPTPartitionEntry {
  pub partition_type_guid: [u8; 16],
  pub unique_partition_guid: [u8; 16],
  pub starting_lba: u64,
  pub ending_lba: u64,
  pub attribute_bits: u64,
  pub partition_name: [u16; 36],
}

#[cfg(feature = "diskutils")]
// lol thanks wikipedia
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct GPTHeader {
  pub magic: [u8; 8],
  pub revision: u32,
  pub header_size: u32,
  pub header_crc32: u32,
  pub reserved0: u32,
  pub current_lba: u64,
  pub backup_lba: u64,
  pub first_usable_lba: u64,
  pub last_usable_lba: u64,
  pub disk_guid: [u8; 16],
  pub partition_entry_lba: u64,
  pub num_partition_entries: u32,
  pub size_partition_entry: u32,
  pub partition_entry_array_crc32: u32,
}
