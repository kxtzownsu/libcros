use libcros::libargs::ArgCheck;
use libcros::libc::mkdir;

fn main() {
  let mut args = ArgCheck::new();
  let flags_path = args.fequals_str("--path", "-p", "Which path to make");

  let path: Vec<u8> = if flags_path.is_empty() {
    b"/tmp/wow\0".to_vec()
  } else {
    let mut v = flags_path.as_bytes().to_vec();
    if !v.ends_with(&[0]) {
        v.push(0);
    }
    v
  };


  let rc = unsafe {
    mkdir(path.as_ptr(), 0o755)
  };


  /* -17 == EEXIST */
  if rc != 0  && rc != -17{
    let err = std::io::Error::last_os_error();
    eprintln!("mkdir failed: {}", err);
  } else {
    let end = path.iter().position(|&b| b == 0).unwrap_or(path.len());
    let path_str = std::str::from_utf8(&path[..end]).unwrap();
    println!("created {}", path_str);
  }
}