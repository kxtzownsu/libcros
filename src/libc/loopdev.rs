use std::{ffi::CString, os::unix::ffi::OsStrExt, path::Path};

use super::{close::close, ioctl::ioctl, open::open};

const O_RDWR: usize = 2;
const LOOP_CTL_GET_FREE: usize = 0x4C82;
const LOOP_SET_FD: usize = 0x4C00;
const LOOP_CLR_FD: usize = 0x4C01;

pub struct LoopDevice {
  fd: i32,
  path: String,
}

impl LoopDevice {
  pub fn attach(image: &Path) -> Result<Self, String> {
    let ctrl_path = CString::new("/dev/loop-control").unwrap();
    let ctrl_fd = unsafe { open(ctrl_path.as_ptr() as *const u8, O_RDWR) };
    if ctrl_fd < 0 {
      return Err(format!("open /dev/loop-control failed (rc={})", ctrl_fd));
    }
    let ctrl_fd = ctrl_fd as i32;

    let free_num = unsafe { ioctl(ctrl_fd, LOOP_CTL_GET_FREE, 0) };
    unsafe { close(ctrl_fd) };
    if free_num < 0 {
      return Err(format!("LOOP_CTL_GET_FREE failed (rc={})", free_num));
    }

    let loop_path = format!("/dev/loop{}", free_num);
    let loop_cstr = CString::new(loop_path.as_str()).unwrap();
    let loop_fd = unsafe { open(loop_cstr.as_ptr() as *const u8, O_RDWR) };
    if loop_fd < 0 {
      return Err(format!("open {} failed (rc={})", loop_path, loop_fd));
    }
    let loop_fd = loop_fd as i32;

    let img_cstr = CString::new(image.as_os_str().as_bytes()).map_err(|e| e.to_string())?;
    let img_fd = unsafe { open(img_cstr.as_ptr() as *const u8, O_RDWR) };
    if img_fd < 0 {
      unsafe { close(loop_fd) };
      return Err(format!("open image failed (rc={})", img_fd));
    }
    let img_fd = img_fd as i32;

    let r = unsafe { ioctl(loop_fd, LOOP_SET_FD, img_fd as usize) };
    unsafe { close(img_fd) };
    if r < 0 {
      unsafe { close(loop_fd) };
      return Err(format!("LOOP_SET_FD failed (rc={})", r));
    }

    Ok(Self {
      fd: loop_fd,
      path: loop_path,
    })
  }

  pub fn path(&self) -> &str {
    &self.path
  }
}

impl Drop for LoopDevice {
  fn drop(&mut self) {
    unsafe {
      ioctl(self.fd, LOOP_CLR_FD, 0);
      close(self.fd);
    }
  }
}
