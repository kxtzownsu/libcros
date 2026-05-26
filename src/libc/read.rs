#[cfg(target_arch = "aarch64")]
use crate::libc::asm::aarch64::{syscall3, SYSCALL_READ};
#[cfg(target_arch = "arm")]
use crate::libc::asm::armv7::{syscall3, SYSCALL_READ};
#[cfg(target_arch = "x86_64")]
use crate::libc::asm::x86_64::{syscall3, SYSCALL_READ};

pub unsafe fn read(fd: i32, buf: *mut u8, count: usize) -> isize {
  unsafe { syscall3(SYSCALL_READ, fd as usize, buf as usize, count) }
}
