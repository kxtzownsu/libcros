#[cfg(target_arch = "arm")]
use crate::libc::asm::armv7::{syscall1, SYSCALL_UNLINK};
#[cfg(target_arch = "x86_64")]
use crate::libc::asm::x86_64::{syscall1, SYSCALL_UNLINK};
#[cfg(target_arch = "aarch64")]
use crate::libc::{
  asm::aarch64::{syscall3, SYSCALL_UNLINKAT},
  AT_FDCWD,
};

pub unsafe fn unlink(path: *const u8) -> isize {
  #[cfg(target_arch = "aarch64")]
  unsafe {
    syscall3(SYSCALL_UNLINKAT, AT_FDCWD as usize, path as usize, 0)
  }

  #[cfg(not(target_arch = "aarch64"))]
  unsafe {
    syscall1(SYSCALL_UNLINK, path as usize)
  }
}
