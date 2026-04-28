use std::fs::File;
use std::io::Read;

/// Read /proc/cmdline and return it as a String.
/// Returns an empty string if the file cannot be read.
pub fn read_cmdline() -> String {
  let mut f = match File::open("/proc/cmdline") {
    Ok(f) => f,
    Err(_) => return String::new(),
  };

  let mut buf = String::new();
  match f.read_to_string(&mut buf) {
    Ok(_) => buf.trim().to_string(),
    Err(_) => String::new(),
  }
}

/// Get the value of a key from /proc/cmdline.
/// 
/// Example:
/// `/proc/cmdline`: "root=/dev/sda cros_debug"
/// `get_cmdline_value("root")` returns "/dev/sda"
/// 
/// Returns an empty string if:
/// - the key is not present (e.g: cros_secure isn't above, we would return an empty string)
/// - the key exists but has no value (e.g. cros_debug would return an empty string but `root` wouldn't since it has a value)
pub fn get_cmdline_value(key: &str) -> String {
  let cmdline = read_cmdline();

  for part in cmdline.split_whitespace() {
    if let Some(pos) = part.find('=') {
      let k = &part[..pos];
      let v = &part[pos + 1..];

      if k == key {
        return v.to_string();
      }
    }
  }

  String::new()
}