pub const SYSCALL_IOCTL: usize = 29;
pub const SYSCALL_MKDIRAT: usize = 34;
pub const SYSCALL_UNLINKAT: usize = 35;
pub const SYSCALL_UMOUNT2: usize = 39;
pub const SYSCALL_MOUNT: usize = 40;
pub const SYSCALL_FTRUNCATE: usize = 46;
pub const SYSCALL_CHDIR: usize = 49;
pub const SYSCALL_CHROOT: usize = 51;
pub const SYSCALL_OPENAT: usize = 56;
pub const SYSCALL_CLOSE: usize = 57;
pub const SYSCALL_LSEEK: usize = 62;
pub const SYSCALL_READ: usize = 63;
pub const SYSCALL_WRITE: usize = 64;
pub const SYSCALL_EXIT: usize = 93;
pub const SYSCALL_GETPID: usize = 172;
pub const SYSCALL_GETUID: usize = 174;
pub const SYSCALL_GETEUID: usize = 175;
pub const SYSCALL_EXECVE: usize = 221;
pub const SYSCALL_WAIT4: usize = 260;
pub const SYSCALL_GETCWD: usize = 17;

pub unsafe fn syscall0(n: usize) -> isize {
  let ret: isize;
  unsafe {
    core::arch::asm!(
      "svc #0",
      in("x8") n,
      lateout("x0") ret,
      options(nostack),
    );
  }
  ret
}

pub unsafe fn syscall1(n: usize, a1: usize) -> isize {
  let ret: isize;
  unsafe {
    core::arch::asm!(
      "svc #0",
      in("x8") n,
      inlateout("x0") a1 => ret,
      options(nostack),
    );
  }
  ret
}

pub unsafe fn syscall2(n: usize, a1: usize, a2: usize) -> isize {
  let ret: isize;
  unsafe {
    core::arch::asm!(
      "svc #0",
      in("x8") n,
      inlateout("x0") a1 => ret,
      in("x1") a2,
      options(nostack),
    );
  }
  ret
}

pub unsafe fn syscall3(n: usize, a1: usize, a2: usize, a3: usize) -> isize {
  let ret: isize;
  unsafe {
    core::arch::asm!(
      "svc #0",
      in("x8") n,
      inlateout("x0") a1 => ret,
      in("x1") a2,
      in("x2") a3,
      options(nostack),
    );
  }
  ret
}

pub unsafe fn syscall4(n: usize, a1: usize, a2: usize, a3: usize, a4: usize) -> isize {
  let ret: isize;
  unsafe {
    core::arch::asm!(
      "svc #0",
      in("x8") n,
      inlateout("x0") a1 => ret,
      in("x1") a2,
      in("x2") a3,
      in("x3") a4,
      options(nostack),
    );
  }
  ret
}

pub unsafe fn syscall5(n: usize, a1: usize, a2: usize, a3: usize, a4: usize, a5: usize) -> isize {
  let ret: isize;
  unsafe {
    core::arch::asm!(
      "svc #0",
      in("x8") n,
      inlateout("x0") a1 => ret,
      in("x1") a2,
      in("x2") a3,
      in("x3") a4,
      in("x4") a5,
      options(nostack),
    );
  }
  ret
}
