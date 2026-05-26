#[cfg(target_arch = "aarch64")]
use crate::libc::asm::aarch64::{syscall2, SYSCALL_CHROOT};
#[cfg(target_arch = "arm")]
use crate::libc::asm::armv7::{syscall2, SYSCALL_CHROOT};
#[cfg(target_arch = "x86_64")]
use crate::libc::asm::x86_64::{syscall2, SYSCALL_CHROOT};

pub unsafe fn chroot(path: *const u8) -> isize {
  unsafe { syscall2(SYSCALL_CHROOT, path as usize, 0) }
}
