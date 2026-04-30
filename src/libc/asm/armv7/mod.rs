pub const SYSCALL_READ: usize = 3;
pub const SYSCALL_WRITE: usize = 4;
pub const SYSCALL_OPEN: usize = 5;
pub const SYSCALL_CLOSE: usize = 6;
pub const SYSCALL_MOUNT: usize = 21;
pub const SYSCALL_MKDIR: usize = 39;
pub const SYSCALL_UMOUNT2: usize = 52;
pub const SYSCALL_MKDIRAT: usize = 323;

pub unsafe fn syscall2(n: usize, a1: usize, a2: usize) -> isize {
  let ret: isize;
  core::arch::asm!(
    "svc 0",
    in("r7") n,
    inlateout("r0") a1 => ret,
    in("r1") a2,
    options(nostack),
  );
  ret
}

pub unsafe fn syscall3(n: usize, a1: usize, a2: usize, a3: usize) -> isize {
  let ret: isize;
  core::arch::asm!(
    "svc 0",
    in("r7") n,
    inlateout("r0") a1 => ret,
    in("r1") a2,
    in("r2") a3,
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
    "svc 0",
    in("r7") n,
    inlateout("r0") a1 => ret,
    in("r1") a2,
    in("r2") a3,
    in("r3") a4,
    in("r4") a5,
    options(nostack),
  );
  ret
}