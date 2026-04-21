#![allow(dead_code)]

use std::io::{self, Write};

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
          if ch.is_ascii_alphabetic() {
            break;
          }
        }
      }
    } else {
      out.push(c);
    }
  }
  std::borrow::Cow::Owned(out)
}

pub fn char_display_width(c: char) -> usize {
  match c as u32 {
    /* control characters are zero-width */
    0x0000..=0x001F | 0x007F => 0,
    /* wide (East Asian) ranges */
    0x1100..=0x115F
    | 0x2329..=0x232A
    | 0x2E80..=0x303E
    | 0x3040..=0xA4CF
    | 0xA960..=0xA97F
    | 0xAC00..=0xD7FF
    | 0xF900..=0xFAFF
    | 0xFE10..=0xFE19
    | 0xFE30..=0xFE6F
    | 0xFF01..=0xFF60
    | 0xFFE0..=0xFFE6
    | 0x1B000..=0x1B0FF
    | 0x1F300..=0x1F64F
    | 0x1F900..=0x1FAFF
    | 0x20000..=0x2FFFF
    | 0x30000..=0x3FFFF => 2,
    _ => 1,
  }
}

pub fn str_display_width(s: &str) -> usize {
  s.chars().map(char_display_width).sum()
}

/// Draw centered text in a box.
pub fn box_draw(text: &str) {
  let margin = 5;
  let lines: Vec<&str> = text.split('\n').collect();

  let mut max_len = 0;
  for line in &lines {
    let stripped = strip_ansi(line);
    let visual_width = str_display_width(stripped.as_ref());
    if visual_width > max_len {
      max_len = visual_width;
    }
  }
  max_len += margin * 2;

  println!("┌{}┐", "─".repeat(max_len));
  for line in &lines {
    let stripped = strip_ansi(line);
    let visual_width = str_display_width(stripped.as_ref());
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
