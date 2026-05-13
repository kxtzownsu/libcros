#[cfg(target_arch = "x86_64")]
use crate::libc::asm::x86_64::{syscall2, SYSCALL_UMOUNT2};

#[cfg(target_arch = "aarch64")]
use crate::libc::asm::aarch64::{syscall2, SYSCALL_UMOUNT2};

#[cfg(target_arch = "arm")]
use crate::libc::asm::armv7::{syscall2, SYSCALL_UMOUNT2};

pub unsafe fn umount2(target: *const u8, flags: usize) -> isize {
  unsafe { syscall2(SYSCALL_UMOUNT2, target as usize, flags) }
}
