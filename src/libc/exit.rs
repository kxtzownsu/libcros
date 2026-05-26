#[cfg(target_arch = "aarch64")]
use crate::libc::asm::aarch64::{syscall1, SYSCALL_EXIT};
#[cfg(target_arch = "arm")]
use crate::libc::asm::armv7::{syscall1, SYSCALL_EXIT};
#[cfg(target_arch = "x86_64")]
use crate::libc::asm::x86_64::{syscall1, SYSCALL_EXIT};

pub unsafe fn exit(code: i32) -> ! {
  unsafe { syscall1(SYSCALL_EXIT, code as usize) };
  loop {}
}
