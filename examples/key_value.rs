use libcros::{LOG, LOG_DBG, key_types, keys, kv_get, kv_set, libargs::ArgCheck};

fn main() {
  let mut args: ArgCheck = ArgCheck::new();
  let _verbose: bool = args.fbool("--verbose", "", "Enable debug messages");
  let mut value = args.fequals_str("--value", "-f", "Value of the key");

  args.check_help();

  if value.is_empty() {
    value = "Hello, World!".to_string();
  }

  LOG!(
    "value of {}: {:?}",
    keys::EXAMPLE,
    kv_get(key_types::STRING, keys::EXAMPLE)
  );

  LOG_DBG!("setting {} to {}", keys::EXAMPLE, value);
  kv_set(keys::EXAMPLE, value);

  LOG!(
    "value of {}: {:?}",
    keys::EXAMPLE,
    kv_get(key_types::STRING, keys::EXAMPLE)
  );
}
