#[cfg(target_arch = "aarch64")]
use crate::libc::asm::aarch64::{syscall3, SYSCALL_EXECVE};
#[cfg(target_arch = "arm")]
use crate::libc::asm::armv7::{syscall3, SYSCALL_EXECVE};
#[cfg(target_arch = "x86_64")]
use crate::libc::asm::x86_64::{syscall3, SYSCALL_EXECVE};

pub unsafe fn execve(path: *const u8, argv: *const *const u8, envp: *const *const u8) -> isize {
  unsafe { syscall3(SYSCALL_EXECVE, path as usize, argv as usize, envp as usize) }
}
