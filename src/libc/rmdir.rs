#[cfg(target_arch = "arm")]
use crate::libc::asm::armv7::{syscall1, SYSCALL_RMDIR};
#[cfg(target_arch = "x86_64")]
use crate::libc::asm::x86_64::{syscall1, SYSCALL_RMDIR};
#[cfg(target_arch = "aarch64")]
use crate::libc::{
  asm::aarch64::{syscall3, SYSCALL_UNLINKAT},
  AT_FDCWD,
};

#[cfg(target_arch = "aarch64")]
const AT_REMOVEDIR: usize = 0x200;

pub unsafe fn rmdir(path: *const u8) -> isize {
  #[cfg(target_arch = "aarch64")]
  unsafe {
    syscall3(
      SYSCALL_UNLINKAT,
      AT_FDCWD as usize,
      path as usize,
      AT_REMOVEDIR,
    )
  }

  #[cfg(not(target_arch = "aarch64"))]
  unsafe {
    syscall1(SYSCALL_RMDIR, path as usize)
  }
}
