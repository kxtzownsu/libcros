pub const SYSCALL_READ: usize = 3;
pub const SYSCALL_WRITE: usize = 4;
pub const SYSCALL_OPEN: usize = 5;
pub const SYSCALL_CLOSE: usize = 6;
pub const SYSCALL_UNLINK: usize = 10;
pub const SYSCALL_EXECVE: usize = 11;
pub const SYSCALL_CHDIR: usize = 12;
pub const SYSCALL_LSEEK: usize = 19;
pub const SYSCALL_GETPID: usize = 20;
pub const SYSCALL_MOUNT: usize = 21;
pub const SYSCALL_GETUID: usize = 24;
pub const SYSCALL_MKDIR: usize = 39;
pub const SYSCALL_RMDIR: usize = 40;
pub const SYSCALL_GETEUID: usize = 49;
pub const SYSCALL_UMOUNT2: usize = 52;
pub const SYSCALL_IOCTL: usize = 54;
pub const SYSCALL_CHROOT: usize = 61;
pub const SYSCALL_FTRUNCATE: usize = 93;
pub const SYSCALL_WAIT4: usize = 114;
pub const SYSCALL_GETCWD: usize = 183;
pub const SYSCALL_MKDIRAT: usize = 323;
pub const SYSCALL_EXIT: usize = 1;

pub unsafe fn syscall0(n: usize) -> isize {
  let ret: isize;
  unsafe {
    core::arch::asm!(
      "svc 0",
      in("r7") n,
      lateout("r0") ret,
      options(nostack),
    );
  }
  ret
}

pub unsafe fn syscall1(n: usize, a1: usize) -> isize {
  let ret: isize;
  unsafe {
    core::arch::asm!(
      "svc 0",
      in("r7") n,
      inlateout("r0") a1 => ret,
      options(nostack),
    );
  }
  ret
}

pub unsafe fn syscall2(n: usize, a1: usize, a2: usize) -> isize {
  let ret: isize;
  unsafe {
    core::arch::asm!(
      "svc 0",
      in("r7") n,
      inlateout("r0") a1 => ret,
      in("r1") a2,
      options(nostack),
    );
  }
  ret
}

pub unsafe fn syscall3(n: usize, a1: usize, a2: usize, a3: usize) -> isize {
  let ret: isize;
  unsafe {
    core::arch::asm!(
      "svc 0",
      in("r7") n,
      inlateout("r0") a1 => ret,
      in("r1") a2,
      in("r2") a3,
      options(nostack),
    );
  }
  ret
}

pub unsafe fn syscall4(n: usize, a1: usize, a2: usize, a3: usize, a4: usize) -> isize {
  let ret: isize;
  unsafe {
    core::arch::asm!(
      "svc 0",
      in("r7") n,
      inlateout("r0") a1 => ret,
      in("r1") a2,
      in("r2") a3,
      in("r3") a4,
      options(nostack),
    );
  }
  ret
}

pub unsafe fn syscall5(n: usize, a1: usize, a2: usize, a3: usize, a4: usize, a5: usize) -> isize {
  let ret: isize;
  unsafe {
    core::arch::asm!(
      "svc 0",
      in("r7") n,
      inlateout("r0") a1 => ret,
      in("r1") a2,
      in("r2") a3,
      in("r3") a4,
      in("r4") a5,
      options(nostack),
    );
  }
  ret
}
