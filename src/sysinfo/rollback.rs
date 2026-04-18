use crate::LOG_DBG;

#[cfg(feature = "tlcl")]
use crate::tlcl::TlclRead;

const FIRMWARE_ROLLBACK_NV_INDEX: u32 = 0x1007;
const KERNEL_ROLLBACK_NV_INDEX: u32 = 0x1008;
const FIRMWARE_MANAGEMENT_PARAMETERS_NV_INDEX: u32 = 0x100A;

/// Macro to reduce boilerplate
#[cfg(feature = "tlcl")]
macro_rules! tpm_nv_read {
  ($nv_index:expr, $size:expr, $parse_func:expr) => {{
    let mut outbuf: [u8; $size] = unsafe { core::mem::zeroed() };

    let rc = TlclRead(
      $nv_index,
      outbuf.as_mut_ptr() as *mut core::ffi::c_void,
      $size as u32
    );

    /* Did the TPM return an error? */
    if rc != crate::tlcl::constants::TPM_SUCCESS {
      LOG_DBG!(
        "TlclRead(0x{:X}, outbuf, 0x{:X}) failed with code 0x{:X}",
        $nv_index,
        $size,
        rc
      );
      return u32::MAX; // -1
    }

    /* Is the response smaller than the size we expected? */
    if outbuf.len() < $size {
      LOG_DBG!(
        "TlclRead(0x{:x}, outbuf, 0x{:x}) returned too few bytes (expected 0x{:x}, got 0x{:x})",
        $nv_index,
        $size,
        $size,
        outbuf.len()
      );
      return u32::MAX; // -1
    }

    let result = $parse_func(&outbuf);

    result
  }
}}

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