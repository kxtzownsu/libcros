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

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct GPTPartitionEntry {
  pub partition_type_guid: [u8; 16],
  pub unique_partition_guid: [u8; 16],
  pub starting_lba: u64,
  pub ending_lba: u64,
  pub attribute_bits: u64,
  pub partition_name: [u16; 36],
}

// lol thanks wikipedia
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
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

#[derive(Debug)]
pub struct SysinfoRollbackVersionResponse {
  pub rc: u32,
  pub rollback_version: u32,
}

pub fn write_struct<T>(value: &T, out: &mut [u8]) {
  let size = core::mem::size_of::<T>();
  let bytes = size.min(out.len());

  unsafe {
    core::ptr::copy_nonoverlapping(value as *const T as *const u8, out.as_mut_ptr(), bytes);
  }
}

pub fn read_struct<T: Copy>(input: &[u8]) -> T {
  assert!(input.len() >= core::mem::size_of::<T>());
  unsafe { core::ptr::read_unaligned(input.as_ptr() as *const T) }
}
