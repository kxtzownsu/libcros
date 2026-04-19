use crate::LOG_DBG;

#[cfg(feature = "tlcl")]
use crate::{tlcl::TlclRead, tpm_nv_read};

const FIRMWARE_ROLLBACK_NV_INDEX: u32 = 0x1007;
const KERNEL_ROLLBACK_NV_INDEX: u32 = 0x1008;
const FIRMWARE_MANAGEMENT_PARAMETERS_NV_INDEX: u32 = 0x100A;

/// Fetch the active kernel rollback version from the TPM.
/// Returns u32::MAX (0xFFFFFFFF) on error.
#[cfg(feature = "tlcl")]
pub fn get_kernel_rollback_version() -> u32 {
  tpm_nv_read!(KERNEL_ROLLBACK_NV_INDEX, 0x9, |buf: &[u8]| {
    /*
      OldUI and NewUI have different kernel version formats.
      The first byte determines which one to use.
    */
    match buf[0] {
      0x02 => u32::from_le_bytes(buf[0x5..0x9].try_into().unwrap()),
      0x10 => u32::from_le_bytes(buf[0x4..0x8].try_into().unwrap()),
      _ => u32::MAX,
    }
  })
}

/// Fetch the active firmware rollback version from the TPM.
/// Returns u32::MAX (0xFFFFFFFF) on error.
#[cfg(feature = "tlcl")]
pub fn get_firmware_rollback_version() -> u32 {
  tpm_nv_read!(FIRMWARE_ROLLBACK_NV_INDEX, 0x6, |buf: &[u8]| {
    u32::from_le_bytes(buf[0x2..0x6].try_into().unwrap())
  })
}

/// Fetch the firmware management parameters from the TPM.
/// Returns u32::MAX (0xFFFFFFFF) on error.
#[cfg(feature = "tlcl")]
pub fn get_firmware_management_parameters() -> u32 {
  tpm_nv_read!(FIRMWARE_MANAGEMENT_PARAMETERS_NV_INDEX, 0x8, |buf: &[u8]| {
    u32::from_le_bytes(buf[0x4..0x8].try_into().unwrap())
  })
}