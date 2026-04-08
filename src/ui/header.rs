use regex::Regex;
use unicode_width::UnicodeWidthStr;

use crate::ui::utils::box_draw;

pub fn format_header(splash: &str, additional_text: &str) -> String {
  let re = Regex::new(r"\x1b\[[0-9;]*m").unwrap();
  let max_width = splash
    .split('\n')
    .map(|line| UnicodeWidthStr::width(re.replace_all(line, "").as_ref()))
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

pub fn ui_header(splash: &str, additional_text: &str) {
  box_draw(format_header(splash, additional_text).trim_end());
}
