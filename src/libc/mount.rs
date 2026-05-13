#[cfg(target_arch = "x86_64")]
use crate::libc::asm::x86_64::{syscall5, SYSCALL_MOUNT};

#[cfg(target_arch = "aarch64")]
use crate::libc::asm::aarch64::{syscall5, SYSCALL_MOUNT};

#[cfg(target_arch = "arm")]
use crate::libc::asm::armv7::{syscall5, SYSCALL_MOUNT};

pub mod flags {
  pub const MS_RDONLY: u32 = 1;          /* Mount read-only */
  pub const MS_NOSUID: u32 = 2;          /* Ignore suid and sgid bits */
  pub const MS_NODEV: u32 = 4;           /* Disallow access to device special files */
  pub const MS_NOEXEC: u32 = 8;          /* Disallow program execution */
  pub const MS_SYNCHRONOUS: u32 = 16;    /* Writes are synced at once */
  pub const MS_REMOUNT: u32 = 32;        /* Alter flags of a mounted FS */
  pub const MS_MANDLOCK: u32 = 64;       /* Allow mandatory locks on an FS */
  pub const MS_DIRSYNC: u32 = 128;       /* Directory modifications are synchronous */
  pub const MS_NOSYMFOLLOW: u32 = 256;   /* Do not follow symlinks */

  pub const MS_NOATIME: u32 = 1024;      /* Do not update access times. */
  pub const MS_NODIRATIME: u32 = 2048;   /* Do not update directory access times */

  pub const MS_BIND: u32 = 4096;
  pub const MS_MOVE: u32 = 8192;
  pub const MS_REC: u32 = 16384;

  pub const MS_VERBOSE: u32 = 32768;     /* War is peace. Verbosity is silence.
                                            MS_VERBOSE is deprecated. */

  pub const MS_SILENT: u32 = 32768;

  pub const MS_POSIXACL: u32 = 1 << 16;  /* VFS does not apply the umask */
  pub const MS_UNBINDABLE: u32 = 1 << 17;/* change to unbindable */
  pub const MS_PRIVATE: u32 = 1 << 18;   /* change to private */
  pub const MS_SLAVE: u32 = 1 << 19;     /* change to slave */
  pub const MS_SHARED: u32 = 1 << 20;    /* change to shared */
  pub const MS_RELATIME: u32 = 1 << 21;  /* Update atime relative to mtime/ctime. */
  pub const MS_KERNMOUNT: u32 = 1 << 22; /* this is a kern_mount call */
  pub const MS_I_VERSION: u32 = 1 << 23; /* Update inode I_version field */
  pub const MS_STRICTATIME: u32 = 1 << 24; /* Always perform atime updates */
  pub const MS_LAZYTIME: u32 = 1 << 25;  /* Update the on-disk [acm]times lazily */
}

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