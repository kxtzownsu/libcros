use libcros::{
  LOG,
  ui::{header::ui_header, menu::selection_menu, *},
};

use std::io::Write;

const SPLASH: &str = concat!(
  r#"example splash here
(can be multi-line)"#
);

fn menu_handler(option: MenuOption) {
  if !option.enabled {
    return;
  }

  match option.text.as_str() {
    "Foobar" => {
      LOG!("selected foobar");
    }

    "Example" => {
      LOG!("selected example");
    }

    "Exit" => {
      std::process::exit(0);
    }

    _ => {
      LOG!("invalid option");
    }
  }

  utils::enter_to_continue();
}

fn main() {
  let mut text = String::new();
  text.push_str("Example additional text line 1\n");
  text.push_str("Example additional text line 2\n");

  loop {
    print!("\x1b[H\x1b[2J");
    std::io::stdout().flush().unwrap();

    ui_header(&SPLASH, &text);

    let options = vec![
      MenuOption::new("Foobar", true, constants::COLOR_RESET),
      MenuOption::new("Example", true, constants::COLOR_GREEN_B),
      MenuOption::new("Disabled", false, constants::COLOR_RESET),
      MenuOption::new("Exit", true, constants::COLOR_RESET),
    ];

    if let Some(choice) = selection_menu(&options) {
      menu_handler(options[choice].clone());
    }
  }
}