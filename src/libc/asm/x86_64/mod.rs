pub const SYSCALL_READ: usize = 0;
pub const SYSCALL_WRITE: usize = 1;
pub const SYSCALL_OPEN: usize = 2;
pub const SYSCALL_CLOSE: usize = 3;
pub const SYSCALL_IOCTL: usize = 16;
pub const SYSCALL_EXECVE: usize = 59;
pub const SYSCALL_FTRUNCATE: usize = 77;
pub const SYSCALL_CHDIR: usize = 80;
pub const SYSCALL_MKDIR: usize = 83;
pub const SYSCALL_CHROOT: usize = 161;
pub const SYSCALL_MOUNT: usize = 165;
pub const SYSCALL_UMOUNT2: usize = 166;
pub const SYSCALL_MKDIRAT: usize = 258;

pub unsafe fn syscall2(n: usize, a1: usize, a2: usize) -> isize {
  let ret: isize;
  unsafe { core::arch::asm!(
    "syscall",
    inlateout("rax") n => ret,
    in("rdi") a1,
    in("rsi") a2,
    lateout("rcx") _,
    lateout("r11") _,
    options(nostack),
  ) };
  ret
}

pub unsafe fn syscall3(n: usize, a1: usize, a2: usize, a3: usize) -> isize {
  let ret: isize;
  unsafe { core::arch::asm!(
    "syscall",
    inlateout("rax") n => ret,
    in("rdi") a1,
    in("rsi") a2,
    in("rdx") a3,
    lateout("rcx") _,
    lateout("r11") _,
    options(nostack),
  ) };
  ret
}

pub unsafe fn syscall5(
  n: usize,
  a1: usize,
  a2: usize,
  a3: usize,
  a4: usize,
  a5: usize,
) -> isize {
  let ret: isize;
  unsafe { core::arch::asm!(
    "syscall",
    inlateout("rax") n => ret,
    in("rdi") a1,
    in("rsi") a2,
    in("rdx") a3,
    in("r10") a4,
    in("r8") a5,
    lateout("rcx") _,
    lateout("r11") _,
    options(nostack),
  ) };
  ret
}
