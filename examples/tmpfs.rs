use std::{ffi::CString, mem, path::Path};

use libcros::{
  libargs::ArgCheck,
  libc::{errno::EEXIST, loopdev::LoopDevice, mkdir, mount, tmpfs::mount_tmpfs},
  Logger, LOG, LOG_FATAL,
};

const ROOT_PART: &str = "/dev/loop0p1";
const OVERLAY_PART: &str = "/dev/loop0p2";
const NEWROOT: &str = "/tmp/libcros/newroot";
const OVERLAY_MNT: &str = "/tmp/libcros/overlay-partition";
const RAMDISK: &str = "/tmp/libcros/ramdisk";
const RAMDISK_SIZE: u64 = 64 * 1024 * 1024;

fn do_mkdir(path: &str) {
  let c = CString::new(path).unwrap();
  let r = unsafe { mkdir(c.as_ptr() as *const u8, 0o755) };
  /* -17 == EEXIST */
  if r < 0 && r != -EEXIST {
    LOG_FATAL!("mkdir {} failed (rc={})", path, r);
  }
}

fn do_mount(source: &str, target: &str, fstype: &str, flags: usize, data: Option<&str>) {
  let src = CString::new(source).unwrap();
  let tgt = CString::new(target).unwrap();
  let fst = CString::new(fstype).unwrap();

  let data_ptr;
  let data_cstr;

  if let Some(d) = data {
    data_cstr = CString::new(d).unwrap();
    data_ptr = data_cstr.as_ptr();
  } else {
    data_ptr = std::ptr::null();
  }

  let r = unsafe {
    mount(
      src.as_ptr() as *const u8,
      tgt.as_ptr() as *const u8,
      fst.as_ptr() as *const u8,
      flags,
      data_ptr as *const u8,
    )
  };

  if r < 0 {
    LOG_FATAL!(
      "mount {} -> {} ({}) failed (rc={})",
      source,
      target,
      fstype,
      r
    );
  }
}

fn main() {
  let mut args = ArgCheck::new();
  let verbose = args.fbool("--verbose", "", "Enable debug messages");
  args.check_help();
  Logger::init(verbose, true);

  /* mount the root partition */
  do_mkdir(NEWROOT);
  do_mount(
    ROOT_PART,
    NEWROOT,
    "ext4",
    libcros::libc::mount::flags::MS_RDONLY as usize,
    None,
  );
  LOG!("mounted {} -> {}", ROOT_PART, NEWROOT);

  /* mount the overlay partition */
  do_mkdir(OVERLAY_MNT);
  do_mount(OVERLAY_PART, OVERLAY_MNT, "vfat", 0, None);
  LOG!("mounted {} -> {}", OVERLAY_PART, OVERLAY_MNT);

  /* TODO: verification stuff goes here */

  /* set up ramdisk, copy overlay image into it, attach a loop device */
  do_mkdir(RAMDISK);
  mount_tmpfs(RAMDISK, RAMDISK_SIZE)
    .unwrap_or_else(|e| LOG_FATAL!("mount tmpfs at {} failed: {}", RAMDISK, e));
  LOG!("tmpfs at {} ({} MiB)", RAMDISK, RAMDISK_SIZE / 1024 / 1024);

  std::fs::copy(
    format!("{}/overlay.img", OVERLAY_MNT),
    format!("{}/overlay.img", RAMDISK),
  )
  .unwrap_or_else(|e| LOG_FATAL!("copy overlay.img failed: {}", e));
  LOG!("copied overlay.img to {}", RAMDISK);

  let loop_dev = LoopDevice::attach(Path::new(&format!("{}/overlay.img", RAMDISK)))
    .unwrap_or_else(|e| LOG_FATAL!("loop attach failed: {}", e));
  LOG!("overlay loop device: {}", loop_dev.path());

  /*
    mount the loop device's filesystem somewhere so overlayfs can use it as
    a directory layer, then mount overlayfs onto /newroot.

    lowerdir ordering: rightmost = bottom of stack, leftmost = top.
  */

  let overlay_fs_mnt = format!("{}/overlay-fs", RAMDISK);
  let upper = format!("{}/upper", RAMDISK);
  let work = format!("{}/work", RAMDISK);

  do_mkdir(&overlay_fs_mnt);
  do_mount(loop_dev.path(), &overlay_fs_mnt, "ext4", 0, None);

  do_mkdir(&upper);
  do_mkdir(&work);

  let opts = format!(
    "lowerdir={}:{},upperdir={},workdir={}",
    overlay_fs_mnt, NEWROOT, upper, work
  );
  libcros::LOG_DBG!("opts: {:?}", opts);
  do_mount("overlay", NEWROOT, "overlay", 0, Some(&opts));
  LOG!("overlay mounted on {}", NEWROOT);

  /* loop_dev must stay alive for as long as the overlay is mounted.
  leak it here since we're about to exec into init anyway. */
  mem::forget(loop_dev);
}
