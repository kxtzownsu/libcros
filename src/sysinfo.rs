#![allow(unused_assignments)] // for kernver::bytes & fwver::bytes

#[cfg(feature = "tlcl")]
use crate::tlcl::TlclRead;
use crate::LOG_DBG;

/// Fetch the active kernel rollback version from the TPM.
/// Returns u32::MAX (0xFFFFFFFF) on error.
#[cfg(feature = "tlcl")]
pub fn kernver() -> u32 {
  const SIZE: u32 = 0x9;
  const NV_INDEX: u32 = 0x1008;
  let mut bytes: u32 = u32::MAX;
  let mut outbuf: [u8; SIZE as usize] = unsafe { core::mem::zeroed() };

  let rc = TlclRead(
    NV_INDEX,
    outbuf.as_mut_ptr() as *mut core::ffi::c_void,
    SIZE,
  );

  if rc != crate::tlcl::constants::TPM_SUCCESS {
    LOG_DBG!("TlclRead(0x{:x}, outbuf, 0x{:x}) failed with code: 0x{:x}", NV_INDEX, SIZE, rc);
    return u32::MAX;
  }

  if outbuf.len() < SIZE as usize {
    LOG_DBG!("TlclRead(0x{:x}, outbuf, 0x{:x}) returned too few bytes (expected 0x{:x}, got 0x{:x})", NV_INDEX, SIZE, SIZE, outbuf.len());
    return u32::MAX;
  }

  if outbuf[0] == 0x02 {
    bytes = u32::from_le_bytes(outbuf[0x5..0x9].try_into().unwrap());
  } else if outbuf[0] == 0x10 {
    bytes = u32::from_le_bytes(outbuf[0x4..0x8].try_into().unwrap());
  }

  LOG_DBG!("TlclRead(0x{:x}, outbuf, 0x{:x}) returned rc 0x{:x} with data 0x{:x}", NV_INDEX, SIZE, rc, bytes);

  bytes
}

/// Fetch the active firmware rollback version from the TPM.
/// Returns u32::MAX (0xFFFFFFFF) on error.
#[cfg(feature = "tlcl")]
pub fn fwver() -> u32 {
  const SIZE: u32 = 0x6;
  const NV_INDEX: u32 = 0x1007;
  let mut bytes: u32 = u32::MAX;
  let mut outbuf: [u8; SIZE as usize] = unsafe { core::mem::zeroed() };

  let rc = TlclRead(
    NV_INDEX,
    outbuf.as_mut_ptr() as *mut core::ffi::c_void,
    SIZE,
  );

  if rc != crate::tlcl::constants::TPM_SUCCESS {
    LOG_DBG!("TlclRead(0x{:x}, outbuf, 0x{:x}) failed with code: 0x{:x}", NV_INDEX, SIZE, rc);
    return u32::MAX;
  }

  if outbuf.len() < SIZE as usize {
    LOG_DBG!("TlclRead(0x{:x}, outbuf, 0x{:x}) returned too few bytes (expected 0x{:x}, got 0x{:x})", NV_INDEX, SIZE, SIZE, outbuf.len());
    return u32::MAX;
  }

  bytes = u32::from_le_bytes(outbuf[0x2..0x6].try_into().unwrap());

  LOG_DBG!("TlclRead(0x{:x}, outbuf, 0x{:x}) returned rc 0x{:x} with data 0x{:x}", NV_INDEX, SIZE, rc, bytes);

  bytes
}


/* Below this line are stubs for functions that require feature flags. */

#[cfg(not(feature = "tlcl"))]
/// This is a stub for whenever Tlcl isn't enabled. Due to the fact that
/// kernver is from the TPM but we can't fetch from the TPM without the
/// Tlcl library, we just stub it here and return u32::MAX (0xFFFFFFFF)
pub fn kernver() -> u32 {
  LOG_DBG!("tlcl feature not enabled");
  u32::MAX
}

#[cfg(not(feature = "tlcl"))]
/// This is a stub for whenever Tlcl isn't enabled. Due to the fact that
/// fwver is from the TPM but we can't fetch from the TPM without the
/// Tlcl library, we just stub it here and return u32::MAX (0xFFFFFFFF)
pub fn fwver() -> u32 {
  LOG_DBG!("tlcl feature not enabled");
  u32::MAX
}