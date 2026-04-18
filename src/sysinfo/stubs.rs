#![allow(unused_imports)]

use crate::LOG_DBG;

#[cfg(not(feature = "tlcl"))]
/// This is a stub for whenever Tlcl isn't enabled. Due to the fact that
/// the kernel rollback version is in the TPM but we can't fetch from the
/// TPM without the Tlcl library, we just stub it here and return u32::MAX 
// (0xFFFFFFFF)
pub fn get_kernel_rollback_version() -> u32 {
  LOG_DBG!("tlcl feature not enabled");
  u32::MAX
}

#[cfg(not(feature = "tlcl"))]
/// This is a stub for whenever Tlcl isn't enabled. Due to the fact that
/// the firmware rollback version is in the TPM but we can't fetch from the
/// TPM without the Tlcl library, we just stub it here and return u32::MAX 
// (0xFFFFFFFF)
pub fn get_firmware_rollback_version() -> u32 {
  LOG_DBG!("tlcl feature not enabled");
  u32::MAX
}

#[cfg(not(feature = "tlcl"))]
/// This is a stub for whenever Tlcl isn't enabled. Due to the fact that
/// the firmware management parameters is in the TPM but we can't fetch from the
/// TPM without the Tlcl library, we just stub it here and return u32::MAX 
// (0xFFFFFFFF)
pub fn get_firmware_management_parameters() -> u32 {
  LOG_DBG!("tlcl feature not enabled");
  u32::MAX
}