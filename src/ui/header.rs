use unicode_width::UnicodeWidthStr;

use crate::ui::utils::{strip_ansi, box_draw};

/// Build header text.
pub fn format_header(splash: &str, additional_text: &str) -> String {
  let max_width = splash
    .split('\n')
    .map(|line| UnicodeWidthStr::width(strip_ansi(line).as_ref()))
    .max()
    .unwrap_or(0);

  let mut text = String::new();
  text.push_str(splash.trim_end_matches('\n'));
  text.push('\n');
  text.push_str(&"─".repeat(max_width));

  if !additional_text.trim().is_empty() {
    text.push('\n');
    text.push_str(additional_text);
    if !additional_text.ends_with('\n') {
      text.push('\n');
    }
  }

  text
}

/// Draw boxed header.
pub fn ui_header(splash: &str, additional_text: &str) {
  box_draw(format_header(splash, additional_text).trim_end());
}
