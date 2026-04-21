#![allow(dead_code)]

use std::{
  io::{self, Write},
};

use libc::{ECHO, ICANON, STDIN_FILENO, TCSANOW, tcgetattr, tcsetattr, termios};

pub fn strip_ansi(s: &str) -> std::borrow::Cow<'_, str> {
  if !s.contains('\x1b') {
    return std::borrow::Cow::Borrowed(s);
  }
  let mut out = String::with_capacity(s.len());
  let mut chars = s.chars();
  while let Some(c) = chars.next() {
    if c == '\x1b' {
      if chars.next() == Some('[') {
        for ch in chars.by_ref() {
          if ch.is_ascii_alphabetic() { break; }
        }
      }
    } else {
      out.push(c);
    }
  }
  std::borrow::Cow::Owned(out)
}

/// Draw centered text in a box.
pub fn box_draw(text: &str) {
  let margin = 5;
  let lines: Vec<&str> = text.split('\n').collect();

  let mut max_len = 0;
  for line in &lines {
    let stripped = strip_ansi(line);
    let visual_width = unicode_width::UnicodeWidthStr::width(stripped.as_ref());
    if visual_width > max_len {
      max_len = visual_width;
    }
  }
  max_len += margin * 2;

  println!("┌{}┐", "─".repeat(max_len));
  for line in &lines {
    let stripped = strip_ansi(line);
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

/// Read one line from stdin.
pub fn input(prompt: &str) -> String {
  print!("{}", prompt);
  io::stdout().flush().unwrap();

  let mut buffer = String::new();
  io::stdin()
    .read_line(&mut buffer)
    .expect("Failed to read input");
  buffer.trim().to_string()
}

/// Print continue prompt and wait for Enter.
pub fn enter_to_continue() {
  println!("Press ENTER to continue!");
  input("");
}

/// Enable terminal raw mode.
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

/// Restore terminal settings.
pub fn disable_raw_mode(original: libc::termios) {
  unsafe {
    tcsetattr(STDIN_FILENO, TCSANOW, &original);
  }
}
