use libcros::crypto::{crc32, adler32, sha1, sha256, hmac_sha256, pbkdf2_hmac_sha256};
use libcros::{LOG, LOG_DBG, kv_get, kv_set, keys, key_types, keyval::KvValue, libargs::ArgCheck};

fn main() {
  let mut args: ArgCheck = ArgCheck::new();
  let _verbose: bool = args.fbool("--verbose", "", "Enable debug messages");
  let mut value = args.fequals_str("--value", "-f", "Value to pass through to cryptography functions");

  args.check_help();

  if value.is_empty() {
    value = "Hello, World!".to_string();
  }

  /*
  Hello, World!

  expected:
  CRC32: 0xec4ac3d0
  ADLER32: 0x1f9e046a
  SHA1: 0a0a9f2a6772942557ab5355d76af442f8f65e01
  SHA256: dffd6021bb2bd5b0af676290809ec3a53191dd81c7f70a4b28688a362182986f
  HMAC-SHA256 (key="secret"): fcfaffa7fef86515c7beb6b62d779fa4ccf092f2e61c164376054271252821ff 
  PBKDF2-HMAC-SHA256 (salt="saltsalt", iterations=10000, output_len=32): 39143d32b2af92c845798e0c5c3a2f60f491af611240433036bf388a5a38032a
  */

  LOG_DBG!("setting value to {}", value);
  kv_set(keys::EXAMPLE, value);

  // always go through kv_get.
  let data = match kv_get(key_types::STRING, keys::EXAMPLE) {
    Some(KvValue::String(s)) => s,
    _ => panic!("invalid or missing EXAMPLE value"),
  };
  let bytes = data.as_bytes();

  LOG!("CRC32: 0x{:08x}", crc32(bytes));
  LOG!("ADLER32: 0x{:08x}", adler32(bytes));

  let s1 = sha1(bytes);
  LOG!("SHA1: {}", s1.iter().map(|b| format!("{:02x}", b)).collect::<String>());

  let s256 = sha256(bytes);
  LOG!("SHA256: {}", s256.iter().map(|b| format!("{:02x}", b)).collect::<String>());

  let hmac = hmac_sha256(b"secret", bytes);
  LOG!("HMAC-SHA256 (key=\"secret\"): {}", hmac.iter().map(|b| format!("{:02x}", b)).collect::<String>());

  let pbkdf2 = pbkdf2_hmac_sha256(bytes, b"saltsalt", 10000, 32);
  LOG!("PBKDF2-HMAC-SHA256 (salt=\"saltsalt\", iterations=10000, output_len=32): {}", pbkdf2.iter().map(|b| format!("{:02x}", b)).collect::<String>());
}
