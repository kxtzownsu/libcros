use std::io::{self, Read, Write};

use crate::ui::{
  constants::COLOR_RESET,
  utils::{disable_raw_mode, enable_raw_mode},
  MenuOption,
};

/// Arrow-key menu.
/// Returns None on Ctrl-C or input failure.
pub fn selection_menu(options: &[MenuOption]) -> Option<usize> {
  if options.is_empty() {
    return None;
  }

  let mut selected = 0;
  let stdin = io::stdin();
  let mut stdout = io::stdout();

  let original_termios = enable_raw_mode();

  loop {
    print!("\r");
    for (i, option) in options.iter().enumerate() {
      if i == selected {
        print!("{}> {} <{}", option.color_code, option.text, COLOR_RESET);
      } else {
        print!("{}  {}  {}", option.color_code, option.text, COLOR_RESET);
      }
      print!("\n");
    }
    print!("\x1b[J");
    stdout.flush().unwrap();

    let mut buf = [0u8; 1];
    if stdin.lock().read_exact(&mut buf).is_err() {
      break;
    }

    match buf[0] {
      b'\n' | b'\r' => {
        if options[selected].enabled {
          stdout.flush().unwrap();
          disable_raw_mode(original_termios);
          print!("\n");
          return Some(selected);
        }
      }
      0x1b => {
        let mut seq = [0u8; 2];
        if stdin.lock().read_exact(&mut seq).is_ok() && seq[0] == b'[' {
          match seq[1] {
            b'A' => {
              if selected > 0 {
                selected -= 1;
              }
            }
            b'B' => {
              if selected < options.len() - 1 {
                selected += 1;
              }
            }
            _ => {}
          }
        }
      }
      0x03 => {
        disable_raw_mode(original_termios);
        return None;
      }
      _ => {}
    }

    for _ in 0..options.len() {
      print!("\x1b[1A");
    }
    print!("\r");
    stdout.flush().unwrap();
  }

  disable_raw_mode(original_termios);
  None
}
