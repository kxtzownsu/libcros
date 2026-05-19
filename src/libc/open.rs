#[cfg(target_arch = "x86_64")]
use crate::libc::asm::x86_64::{syscall3, SYSCALL_OPEN};

#[cfg(target_arch = "aarch64")]
use crate::libc::asm::aarch64::{syscall5, SYSCALL_OPENAT};

#[cfg(target_arch = "arm")]
use crate::libc::asm::armv7::{syscall3, SYSCALL_OPEN};

pub unsafe fn open(path: *const u8, flags: usize) -> isize {
  #[cfg(not(target_arch = "aarch64"))]
  unsafe {
    syscall3(SYSCALL_OPEN, path as usize, flags, 0)
  }

  #[cfg(target_arch = "aarch64")]
  unsafe {
    syscall5(
      SYSCALL_OPENAT,
      crate::libc::AT_FDCWD as usize,
      path as usize,
      flags,
      0,
      0,
    )
  }
}
