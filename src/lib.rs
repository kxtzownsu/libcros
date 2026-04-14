use std::{
  collections::HashMap,
  sync::{Mutex, OnceLock},
};

static KV: OnceLock<Mutex<HashMap<&'static str, String>>> = OnceLock::new();

fn kv() -> &'static Mutex<HashMap<&'static str, String>> {
  KV.get_or_init(|| Mutex::new(HashMap::new()))
}

/// Set a key/value pair.
pub fn kv_set(key: &'static str, val: impl ToString) {
  kv().lock().unwrap().insert(key, val.to_string());
}

/// Get a value by key.
/// Returns an empty string when missing.
pub fn kv_get(key: &'static str) -> String {
  kv().lock().unwrap().get(key).cloned().unwrap_or_default()
}

/// Get a bool by key.
/// "1" and "true" mean true.
pub fn kv_get_bool(key: &'static str) -> bool {
  matches!(kv_get(key).as_str(), "1" | "true")
}

/// Common keys for the global key/value store.
pub mod keys {
  /// Internal disk path.
  pub const INTERNAL_DISK: &str = "internal_disk";

  #[cfg(feature = "tlcl")]
  /// TPM device path.
  pub const TPM_PATH: &str = "tpm_path";

  #[cfg(feature = "tlcl")]
  #[cfg(feature = "tpm2_0")]
  pub const TPM_TAG: &str = "tpm_tag";

  #[cfg(feature = "tlcl")]
  #[cfg(feature = "tpm2_0")]
  /// Cached physical hierarchy state.
  pub const PH_DISABLED: &str = "ph_disabled";

  #[cfg(feature = "example")]
  pub const EXAMPLE: &str = "example";
}

/// Easy-to-use logging API
pub mod logging;
pub use logging::Logger;

/// Basic cryptography functions
pub mod crypto;

/// Easy-to-use functions to execute commands
pub mod execute;

/// Lightweight argument parser
pub mod libargs;

/// Commonly-used structs
pub mod structs;

/// High-level functions to get misc things from a Chrome device
pub mod sysinfo;

/*
Anything that requires a dependency should be locked behind a feature flag.
*/

#[cfg(feature = "ui")]
pub mod ui;

/*
#[cfg(feature = "diskutils")]
pub mod diskutils;
*/

#[cfg(feature = "tlcl")]
pub mod tlcl;
