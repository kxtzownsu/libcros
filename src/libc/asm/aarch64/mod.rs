pub const SYSCALL_MKDIRAT: usize = 34;
pub const SYSCALL_UMOUNT2: usize = 39;
pub const SYSCALL_MOUNT: usize = 40;
pub const SYSCALL_OPENAT: usize = 56;
pub const SYSCALL_CLOSE: usize = 57;
pub const SYSCALL_READ: usize = 63;
pub const SYSCALL_WRITE: usize = 64;

pub unsafe fn syscall2(n: usize, a1: usize, a2: usize) -> isize {
  let ret: isize;
  core::arch::asm!(
    "svc #0",
    in("x8") n,
    inlateout("x0") a1 => ret,
    in("x1") a2,
    options(nostack),
  );
  ret
}

pub unsafe fn syscall3(n: usize, a1: usize, a2: usize, a3: usize) -> isize {
  let ret: isize;
  core::arch::asm!(
    "svc #0",
    in("x8") n,
    inlateout("x0") a1 => ret,
    in("x1") a2,
    in("x2") a3,
    options(nostack),
  );
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
  ret
}