#![allow(dead_code)]

use std::io::{self, Write};

use libc::{tcgetattr, tcsetattr, termios, ECHO, ICANON, STDIN_FILENO, TCSANOW};
use regex::Regex;

pub fn box_draw(text: &str) {
  let margin = 5;
  let re = Regex::new(r"\x1b\[[0-9;]*m").unwrap();
  let lines: Vec<&str> = text.split('\n').collect();

  let mut max_len = 0;
  for line in &lines {
    let stripped = re.replace_all(line, "");
    let visual_width = unicode_width::UnicodeWidthStr::width(stripped.as_ref());
    if visual_width > max_len {
      max_len = visual_width;
    }
  }
  max_len += margin * 2;

  println!("┌{}┐", "─".repeat(max_len));
  for line in &lines {
    let stripped = re.replace_all(line, "");
    let visual_width = unicode_width::UnicodeWidthStr::width(stripped.as_ref());
    let pad_left = (max_len - visual_width) / 2;
    let pad_right = max_len - visual_width - pad_left;
    println!(
      "│{}{}{}│",
      " ".repeat(pad_left),
      line,
      " ".repeat(pad_right)
    );
  }
  println!("└{}┘", "─".repeat(max_len));
}

pub fn input(prompt: &str) -> String {
  print!("{}", prompt);
  io::stdout().flush().unwrap();

  let mut buffer = String::new();
  io::stdin()
    .read_line(&mut buffer)
    .expect("Failed to read input");
  buffer.trim().to_string()
}

pub fn enter_to_continue() {
  println!("Press ENTER to continue!");
  input("");
}

pub fn enable_raw_mode() -> libc::termios {
  unsafe {
    let mut termios: termios = std::mem::zeroed();
    tcgetattr(STDIN_FILENO, &mut termios);
    let original = termios;

    termios.c_lflag &= !(ICANON | ECHO);
    tcsetattr(STDIN_FILENO, TCSANOW, &termios);

    original
  }
}

pub fn disable_raw_mode(original: libc::termios) {
  unsafe {
    tcsetattr(STDIN_FILENO, TCSANOW, &original);
  }
}
