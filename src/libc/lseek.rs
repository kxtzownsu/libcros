#[cfg(target_arch = "aarch64")]
use crate::libc::asm::aarch64::{syscall3, SYSCALL_LSEEK};
#[cfg(target_arch = "arm")]
use crate::libc::asm::armv7::{syscall3, SYSCALL_LSEEK};
#[cfg(target_arch = "x86_64")]
use crate::libc::asm::x86_64::{syscall3, SYSCALL_LSEEK};

pub unsafe fn lseek(fd: i32, offset: isize, whence: usize) -> isize {
  unsafe { syscall3(SYSCALL_LSEEK, fd as usize, offset as usize, whence) }
}
