#[cfg(target_arch = "x86_64")]
use crate::libc::asm::x86_64::{syscall2, SYSCALL_CLOSE};

#[cfg(target_arch = "aarch64")]
use crate::libc::asm::aarch64::{syscall2, SYSCALL_CLOSE};

#[cfg(target_arch = "arm")]
use crate::libc::asm::armv7::{syscall2, SYSCALL_CLOSE};

pub unsafe fn close(fd: i32) -> isize {
  unsafe { syscall2(SYSCALL_CLOSE, fd as usize, 0) }
}
