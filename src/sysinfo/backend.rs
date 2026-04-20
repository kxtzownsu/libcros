use crate::{LOG_FATAL, kv_erase, kv_set, keys};

#[cfg(feature = "tlcl")]
use crate::tlcl::TlclRead;

/// Macro to reduce boilerplate
#[cfg(feature = "tlcl")]
#[macro_export] macro_rules! tpm_nv_read {
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

const TPM_DEVICES: &[&str] = &[
  "/dev/tpm0",     /* Cr50 & Ti50 */
  "/dev/gsc0",     /* Acropora */
  "/dev/citadel0", /* Nugget-OS */
];
 
/// Opens a global file handle to the GSC device.
/// Tries each device in priority order and stores the first
/// successful handle in the GSC_SOCKET key.
pub fn open_gsc_socket() {
  for path in TPM_DEVICES {
    if let Ok(f) = std::fs::OpenOptions::new().read(true).write(true).open(path) {
      kv_set(keys::GSC_SOCKET, f);
      return;
    }
  }
 
  LOG_FATAL!("failed to open any GSC device");
}
 
/// Closes the global GSC file handle by erasing it from the store.
/// The File is dropped when the key is removed, which closes the fd.
pub fn close_gsc_socket() {
  kv_erase(keys::GSC_SOCKET);
}