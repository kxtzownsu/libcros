#[cfg(target_arch = "aarch64")]
use crate::libc::asm::aarch64::{syscall4, SYSCALL_WAIT4};
#[cfg(target_arch = "arm")]
use crate::libc::asm::armv7::{syscall4, SYSCALL_WAIT4};
#[cfg(target_arch = "x86_64")]
use crate::libc::asm::x86_64::{syscall4, SYSCALL_WAIT4};

pub unsafe fn wait4(pid: i32, status: *mut i32, options: usize, rusage: *mut u8) -> isize {
  unsafe {
    syscall4(
      SYSCALL_WAIT4,
      pid as usize,
      status as usize,
      options,
      rusage as usize,
    )
  }
}
