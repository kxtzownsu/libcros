#[derive(Clone)]
pub struct MenuOption {
  pub color_code: &'static str,
  pub enabled: bool,
  pub text: String,
}

impl MenuOption {
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
