use std::ffi::CString;

use super::mount::mount;

pub fn mount_tmpfs(path: &str, size: u64) -> Result<(), String> {
  let src = CString::new("tmpfs").unwrap();
  let tgt = CString::new(path).map_err(|e| e.to_string())?;
  let fstype = CString::new("tmpfs").unwrap();
  let opts = CString::new(format!("size={}", size)).unwrap();

  let r = unsafe {
    mount(
      src.as_ptr() as *const u8,
      tgt.as_ptr() as *const u8,
      fstype.as_ptr() as *const u8,
      0,
      opts.as_ptr() as *const u8,
    )
  };

  if r < 0 {
    Err(format!("mount syscall returned {}", r))
  } else {
    Ok(())
  }
}
