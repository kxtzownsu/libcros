#[cfg(target_arch = "x86_64")]
use crate::libc::asm::x86_64::{syscall3, SYSCALL_IOCTL};

#[cfg(target_arch = "aarch64")]
use crate::libc::asm::aarch64::{syscall3, SYSCALL_IOCTL};

#[cfg(target_arch = "arm")]
use crate::libc::asm::armv7::{syscall3, SYSCALL_IOCTL};

pub unsafe fn ioctl(fd: i32, request: usize, arg: usize) -> isize {
  unsafe { syscall3(SYSCALL_IOCTL, fd as usize, request, arg) }
}
