pub const AT_FDCWD: isize = -100;

pub mod asm;
pub mod ftruncate;
pub mod ioctl;
pub mod mkdir;
pub mod mount;
pub mod umount;
// pub mod tmpfs;
// pub mod errno;

pub use ftruncate::*;
pub use ioctl::*;
pub use mkdir::*;
pub use mount::*;
pub use umount::*;
// pub use tmpfs::*;
// pub use errno::*;
