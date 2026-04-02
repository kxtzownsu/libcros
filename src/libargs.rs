#![allow(dead_code)]

use std::{collections::HashMap, env};

pub struct ArgCheck {
  pub args: Vec<String>,
  program_name: String,
  description: String,
  flags: HashMap<String, FlagInfo>,
  help_requested: bool,
}

struct FlagInfo {
  long: String,
  short: String,
  description: String,
  takes_value: bool,
}

impl ArgCheck {
  pub fn new() -> Self {
    let args: Vec<String> = env::args().collect();
    let program_name = args
      .get(0)
      .and_then(|p| std::path::Path::new(p).file_name())
      .and_then(|n| n.to_str())
      .unwrap_or("program")
      .to_string();

    let help_requested = args.iter().any(|arg| arg == "--help" || arg == "-h");

    ArgCheck {
      args,
      program_name,
      description: String::new(),
      flags: HashMap::new(),
      help_requested,
    }
  }

  pub fn set_description(&mut self, desc: &str) -> &mut Self {
    self.description = desc.to_string();
    self
  }

  pub fn check_help(&self) {
    if self.help_requested {
      self.show_help();
    }
  }

  pub fn show_help(&self) -> ! {
    println!("Usage: {} [OPTIONS]", self.program_name);

    if !self.description.is_empty() {
      println!("\n{}", self.description);
    }

    if !self.flags.is_empty() {
      println!("\nOPTIONS:");

      let mut flags: Vec<_> = self.flags.values().collect();
      flags.sort_by(|a, b| a.long.cmp(&b.long));

      let max_flag_width = flags
        .iter()
        .map(|flag| {
          let short_len = if !flag.short.is_empty() {
            flag.short.len() + 2
          } else {
            0
          }; /* +2 for ", " */
          let long_len = flag.long.len();
          let value_len = if flag.takes_value { 8 } else { 0 }; /* " <VALUE>" */
          short_len + long_len + value_len
        })
        .max()
        .unwrap_or(0)
        + 10; /* +10 spaces padding */

      for flag in flags {
        let short_part = if !flag.short.is_empty() {
          format!("{}, ", flag.short)
        } else {
          "    ".to_string()
        };

        let value_part = if flag.takes_value { " <VALUE>" } else { "" };

        let flag_part = format!("{}{}{}", short_part, flag.long, value_part);
        println!(
          "  {:<width$} {}",
          flag_part,
          flag.description,
          width = max_flag_width
        );
      }

      println!(
        "  {:<width$} {}",
        "-h, --help",
        "Show this help message",
        width = max_flag_width
      );
    } else {
      println!("\nOPTIONS:");
      println!(
        "  {:<width$} {}",
        "-h, --help",
        "Show this help message",
        width = 20
      );
    }

    std::process::exit(0);
  }

  fn register_flag(&mut self, long: &str, short: &str, desc: &str, takes_value: bool) {
    if !self.flags.contains_key(long) {
      self.flags.insert(
        long.to_string(),
        FlagInfo {
          long: long.to_string(),
          short: short.to_string(),
          description: desc.to_string(),
          takes_value,
        },
      );
    }
  }

  /* Returns the value for a matched flag, like `--parameter foobar` or `--parameter=foobar`. With that example, the function would return "foobar" */
  pub fn fequals(&mut self, arg: &str, shorthand: &str, desc: &str) -> Option<String> {
    self.register_flag(arg, shorthand, desc, true);

    for i in 0..self.args.len() {
      let item = &self.args[i];

      /* Handle `--parameter=value` and `-s=value` formats properly */
      if item.starts_with(arg) || (!shorthand.is_empty() && item.starts_with(shorthand)) {
        if let Some(index) = item.find('=') {
          if &item[..index] == arg || &item[..index] == shorthand {
            return Some(item[index + 1..].to_string());
          }
        }
      }

      /* Handle `--parameter value` format */
      if item == arg || (!shorthand.is_empty() && item == shorthand) {
        return self.args.get(i + 1).cloned();
      }
    }
    None
  }

  /* Checks if a flag is present (`--flag` or `-f`) */
  pub fn fbool(&mut self, arg: &str, shorthand: &str, desc: &str) -> bool {
    self.register_flag(arg, shorthand, desc, false);

    self
      .args
      .iter()
      .any(|item| item == arg || item == shorthand)
  }

  pub fn fequals_str(&mut self, arg: &str, shorthand: &str, desc: &str) -> String {
    self.fequals(arg, shorthand, desc).unwrap_or_default()
  }
}

impl Drop for ArgCheck {
  fn drop(&mut self) {
    self.check_help();
  }
}
