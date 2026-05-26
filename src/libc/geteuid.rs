#[cfg(target_arch = "aarch64")]
use crate::libc::asm::aarch64::{syscall0, SYSCALL_GETEUID};
#[cfg(target_arch = "arm")]
use crate::libc::asm::armv7::{syscall0, SYSCALL_GETEUID};
#[cfg(target_arch = "x86_64")]
use crate::libc::asm::x86_64::{syscall0, SYSCALL_GETEUID};

pub unsafe fn geteuid() -> isize {
  unsafe { syscall0(SYSCALL_GETEUID) }
}
