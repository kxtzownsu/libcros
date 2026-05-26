#[cfg(target_arch = "aarch64")]
use crate::libc::asm::aarch64::{syscall2, SYSCALL_GETCWD};
#[cfg(target_arch = "arm")]
use crate::libc::asm::armv7::{syscall2, SYSCALL_GETCWD};
#[cfg(target_arch = "x86_64")]
use crate::libc::asm::x86_64::{syscall2, SYSCALL_GETCWD};

pub unsafe fn getcwd(buf: *mut u8, size: usize) -> isize {
  unsafe { syscall2(SYSCALL_GETCWD, buf as usize, size) }
}
