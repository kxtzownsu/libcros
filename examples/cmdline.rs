use libcros::cmdline::{read_cmdline, get_cmdline_value};

fn main() {
  let cmdline = read_cmdline();
  let root = get_cmdline_value("root");

  if cmdline.contains("cros_debug") {
    println!("true");
  } else {
    println!("false");
  }

  println!("{}", root);
}
