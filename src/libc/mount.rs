#[cfg(target_arch = "x86_64")]
use crate::libc::asm::x86_64::{syscall5, SYSCALL_MOUNT};

#[cfg(target_arch = "aarch64")]
use crate::libc::asm::aarch64::{syscall5, SYSCALL_MOUNT};

#[cfg(target_arch = "arm")]
use crate::libc::asm::armv7::{syscall5, SYSCALL_MOUNT};

pub unsafe fn mount(
  source: *const u8,
  target: *const u8,
  fstype: *const u8,
  flags: usize,
  data: *const u8,
) -> isize {
  unsafe { syscall5(
    SYSCALL_MOUNT,
    source as usize,
    target as usize,
    fstype as usize,
    flags,
    data as usize,
  ) }
}