use std::{env, ffi::CString, fs, path::Path, ptr};

use libcros::{
  libargs::ArgCheck,
  libc::{
    chdir, chroot, close, execve, getcwd, geteuid, getpid, getuid, lseek, mkdir, mount, open, read,
    rmdir, umount2, unlink, wait4, write,
  },
};

const O_RDWR: usize = 2;
const SEEK_SET: usize = 0;

fn cstr(value: &str) -> CString {
  CString::new(value).unwrap_or_else(|e| panic!("invalid string {:?}: {}", value, e))
}

fn run_chdir(path: &str) {
  let path = cstr(path);
  let rc = unsafe { chdir(path.as_ptr() as *const u8) };
  if rc < 0 {
    println!("chdir failed (rc={})", rc);
    return;
  }

  println!("cwd: {}", env::current_dir().unwrap().display());
}

fn run_getcwd() {
  let mut buf = [0u8; 4096];
  let rc = unsafe { getcwd(buf.as_mut_ptr(), buf.len()) };
  if rc < 0 {
    println!("getcwd failed (rc={})", rc);
    return;
  }

  let len = buf.iter().position(|&b| b == 0).unwrap_or(rc as usize);
  println!("getcwd: {}", String::from_utf8_lossy(&buf[..len]));
}

fn run_ids() -> bool {
  let uid = unsafe { getuid() };
  let euid = unsafe { geteuid() };
  let pid = unsafe { getpid() };
  println!("pid: {}", pid);
  println!("uid: {}", uid);
  println!("euid: {}", euid);
  euid == 0
}

fn run_file_io() {
  let _ = fs::write("/tmp/libcros-libc-example.txt", b"");
  let path = cstr("/tmp/libcros-libc-example.txt");
  let fd = unsafe { open(path.as_ptr() as *const u8, O_RDWR) };
  if fd < 0 {
    println!("open failed (rc={})", fd);
    return;
  }

  let fd = fd as i32;
  let msg = b"libcros syscall test\n";
  let rc = unsafe { write(fd, msg.as_ptr(), msg.len()) };
  println!("write rc: {}", rc);

  let rc = unsafe { lseek(fd, 0, SEEK_SET) };
  println!("lseek rc: {}", rc);

  let mut buf = [0u8; 64];
  let rc = unsafe { read(fd, buf.as_mut_ptr(), buf.len()) };
  if rc < 0 {
    println!("read failed (rc={})", rc);
  } else {
    println!("read: {}", String::from_utf8_lossy(&buf[..rc as usize]));
  }

  unsafe { close(fd) };

  let rc = unsafe { unlink(path.as_ptr() as *const u8) };
  println!("unlink rc: {}", rc);
}

fn run_dir_io() {
  let path = cstr("/tmp/libcros-libc-example-dir");
  let rc = unsafe { mkdir(path.as_ptr() as *const u8, 0o755) };
  println!("mkdir rc: {}", rc);

  let rc = unsafe { rmdir(path.as_ptr() as *const u8) };
  println!("rmdir rc: {}", rc);
}

fn run_wait4() {
  let mut status = 0;
  let rc = unsafe { wait4(-1, &mut status, 1, ptr::null_mut()) };
  println!("wait4 rc: {}", rc);
}

fn run_root_mount_check(is_root: bool) {
  if !is_root {
    println!("mount skipped: euid is not 0");
    return;
  }

  let src = cstr("tmpfs");
  let target = cstr("/tmp/libcros-libc-mount");
  let fstype = cstr("tmpfs");
  let data = cstr("size=4096");
  let _ = fs::create_dir_all("/tmp/libcros-libc-mount");
  let rc = unsafe {
    mount(
      src.as_ptr() as *const u8,
      target.as_ptr() as *const u8,
      fstype.as_ptr() as *const u8,
      0,
      data.as_ptr() as *const u8,
    )
  };
  println!("mount rc: {}", rc);
  if rc == 0 {
    let rc = unsafe { umount2(target.as_ptr() as *const u8, 0) };
    println!("umount2 rc: {}", rc);
  }
}

fn run_chroot(path: &str) {
  let path = cstr(path);
  let rc = unsafe { chroot(path.as_ptr() as *const u8) };
  if rc < 0 {
    println!("chroot failed (rc={})", rc);
    return;
  }

  run_chdir("/");
  println!("chroot ok");
}

fn run_execve(path: &str) {
  let path = cstr(path);
  let arg0 = path.clone();
  let argv = [arg0.as_ptr() as *const u8, ptr::null()];
  let envp = [ptr::null()];
  let rc = unsafe { execve(path.as_ptr() as *const u8, argv.as_ptr(), envp.as_ptr()) };
  println!("execve failed (rc={})", rc);
}

fn warn_chroot_exec(chroot_path: &str, exec_path: &str) {
  if chroot_path.is_empty() || exec_path.is_empty() || !exec_path.starts_with('/') {
    return;
  }

  let path = Path::new(chroot_path).join(exec_path.trim_start_matches('/'));
  if !path.exists() {
    println!(
      "exec warning: {} will resolve to {} after chroot",
      exec_path,
      path.display()
    );
  }
}

fn main() {
  let mut args = ArgCheck::new();
  args.set_description("Test libcros libc syscall wrappers");
  let chdir_path = args.fequals_str("--chdir", "", "Path to pass to chdir");
  let chroot_path = args.fequals_str("--chroot", "", "Path to pass to chroot");
  let exec_path = args.fequals_str("--exec", "", "Path to pass to execve");
  let run_all = args.fbool("--all", "", "Run safe syscall tests");
  let run_mount = args.fbool("--mount", "", "Run root-only mount test");
  args.check_help();

  let is_root = run_ids();

  if run_all {
    run_getcwd();
    run_file_io();
    run_dir_io();
    run_wait4();
  }

  if !chdir_path.is_empty() {
    run_chdir(&chdir_path);
  }

  if !chroot_path.is_empty() {
    warn_chroot_exec(&chroot_path, &exec_path);
    if is_root {
      run_chroot(&chroot_path);
    } else {
      println!("chroot skipped: euid is not 0");
    }
  }

  if run_mount {
    run_root_mount_check(is_root);
  }

  if !exec_path.is_empty() {
    run_execve(&exec_path);
  }
}
