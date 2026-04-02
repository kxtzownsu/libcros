use std::{collections::HashMap, sync::Mutex};

use once_cell::sync::Lazy;

static KV: Lazy<Mutex<HashMap<&'static str, String>>> = Lazy::new(|| Mutex::new(HashMap::new()));

pub fn kv_set(key: &'static str, val: impl ToString) {
  KV.lock().unwrap().insert(key, val.to_string());
}

pub fn kv_get(key: &'static str) -> String {
  KV.lock().unwrap().get(key).cloned().unwrap_or_default()
}

pub fn kv_get_bool(key: &'static str) -> bool {
  matches!(kv_get(key).as_str(), "1" | "true")
}

pub mod keys {
  pub const TPM_PATH: &str = "tpm_path";
  pub const INTERNAL_DISK: &str = "internal_disk";
}

pub mod logging;
pub use logging::Logger;

pub mod structs;

#[cfg(feature = "tlcl")]
pub mod tlcl;
