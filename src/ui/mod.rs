/// Menu entry.
#[derive(Clone)]
pub struct MenuOption {
  /// ANSI color code.
  pub color_code: &'static str,
  /// True if selectable.
  pub enabled: bool,
  /// Display text.
  pub text: String,
}

impl MenuOption {
  /// Create a menu option.
  pub fn new(text: &str, enabled: bool, color_code: &'static str) -> Self {
    Self {
      color_code,
      enabled,
      text: text.to_string(),
    }
  }
}

pub mod constants;
pub mod header;
pub mod menu;
pub mod utils;
