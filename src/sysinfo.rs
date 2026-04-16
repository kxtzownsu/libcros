use crate::LOG_DBG;

#[cfg(feature = "tlcl")]
use crate::tlcl::{TlclRead, TlclGetTPMVersion};

#[cfg(feature = "tlcl")]
const KERNEL_ROLLBACK_NV_INDEX: u32 = 0x1008;
#[cfg(feature = "tlcl")]
const FIRMWARE_ROLLBACK_NV_INDEX: u32 = 0x1007;

/// Macro to reduce boilerplate for TPM NV read operations.
#[cfg(feature = "tlcl")]
macro_rules! tpm_nv_read {
  ($nv_index:expr, $size:expr, $parse_fn:expr) => {{
    let mut outbuf: [u8; $size] = unsafe { core::mem::zeroed() };

    let rc = TlclRead(
      $nv_index,
      outbuf.as_mut_ptr() as *mut core::ffi::c_void,
      $size as u32,
    );

    /* Was there an error when trying to read from the TPM? */
    if rc != crate::tlcl::constants::TPM_SUCCESS {
      if rc == crate::tlcl::constants::TPM_E_BADINDEX {
        LOG_DBG!("NV index 0x{:x} doesn't exist!", $nv_index);
      } else {
        LOG_DBG!(
          "TlclRead(0x{:x}, outbuf, 0x{:x}) failed with code: 0x{:x}",
          $nv_index,
          $size,
          rc
        );
      }
      return u32::MAX;
    }

    /* Is the response the expected size? If not, something went wrong. */
    if outbuf.len() < $size {
      LOG_DBG!(
        "TlclRead(0x{:x}, outbuf, 0x{:x}) returned too few bytes (expected 0x{:x}, got 0x{:x})",
        $nv_index,
        $size,
        $size,
        outbuf.len()
      );
      return u32::MAX;
    }

    /* Parse the buffer. */
    let result = $parse_fn(&outbuf);

    LOG_DBG!(
      "TlclRead(0x{:x}, outbuf, 0x{:x}) returned rc 0x{:x} with data 0x{:x}",
      $nv_index,
      $size,
      rc,
      result
    );

    result
  }};
}

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

#[cfg(feature = "tlcl")]
pub fn get_tpm_version() -> String {
  TlclGetTPMVersion()
}

/* Below this line are stubs for functions that require feature flags. */

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

/// This is a stub for whenever Tlcl isn't enabled. Due to the fact that we
/// can't fetch the TPM version without the Tlcl library, we just stub it here.
#[cfg(not(feature = "tlcl"))]
pub fn get_tpm_version() -> String {
  LOG_DBG!("tlcl feature not enabled");
  "Tlcl was not enabled when compiling libcros.".to_string()
}