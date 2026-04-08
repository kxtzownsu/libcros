#[cfg(feature = "tlcl")]
use crate::tlcl::TlclReadWithOffset;

use crate::LOG_DBG;

/* returns 0xFFFFFFFF on error */
#[cfg(feature = "tlcl")]
pub fn kernver() -> u32 {
  let mut outbuf: [u8; 4] = unsafe { core::mem::zeroed() };

  let rc = TlclReadWithOffset(0x1008, 0x4, 0x5, outbuf.as_mut_ptr() as *mut core::ffi::c_void);
  
  if rc != 0 {
    LOG_DBG!("TlclReadWithOffset failed with code: {}", rc);
    return u32::MAX;
  }

  if outbuf.len() < 4 {
    LOG_DBG!("TlclReadWithOffset returned too few bytes");
    return u32::MAX;
  }

  let bytes: [u8; 4] = outbuf[0..4].try_into().unwrap();
  let val = u32::from_le_bytes(bytes);

  LOG_DBG!("read bytes: {}", val);

  val
}

#[cfg(not(feature = "tlcl"))]
pub fn kernver() -> u32 {
  LOG_DBG!("tlcl feature not enabled");
  u32::MAX
}