#[cfg(target_arch = "x86_64")]
use crate::libc::asm::x86_64::{syscall3, SYSCALL_MKDIR};

#[cfg(target_arch = "aarch64")]
use crate::libc::{
  asm::aarch64::{syscall3, SYSCALL_MKDIRAT},
  AT_FDCWD
};

#[cfg(target_arch = "arm")]
use crate::libc::asm::armv7::{syscall3, SYSCALL_MKDIR};


pub unsafe fn mkdir(path: *const u8, mode: usize) -> isize {
  #[cfg(not(target_arch = "aarch64"))]
  unsafe {
    syscall3(SYSCALL_MKDIR, path as usize, mode, 0)
  }

  #[cfg(target_arch = "aarch64")]
  unsafe {
    syscall3(
      SYSCALL_MKDIRAT,
      AT_FDCWD.try_into().unwrap(),
      path as usize,
      mode,
    )
  }
}