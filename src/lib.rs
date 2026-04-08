use std::{
  collections::HashMap,
  sync::{Mutex, OnceLock},
};

static KV: OnceLock<Mutex<HashMap<&'static str, String>>> = OnceLock::new();

fn kv() -> &'static Mutex<HashMap<&'static str, String>> {
  KV.get_or_init(|| Mutex::new(HashMap::new()))
}

pub fn kv_set(key: &'static str, val: impl ToString) {
  kv().lock().unwrap().insert(key, val.to_string());
}

pub fn kv_get(key: &'static str) -> String {
  kv().lock().unwrap().get(key).cloned().unwrap_or_default()
}

pub fn kv_get_bool(key: &'static str) -> bool {
  matches!(kv_get(key).as_str(), "1" | "true")
}

pub mod keys {
  pub const TPM_PATH: &str = "tpm_path";
  pub const INTERNAL_DISK: &str = "internal_disk";

  #[cfg(feature = "tlcl")]
  #[cfg(feature = "tpm2_0")]
  pub const TPM_TAG: &str = "tpm_tag";

  #[cfg(feature = "tlcl")]
  #[cfg(feature = "tpm2_0")]
  pub const PH_DISABLED: &str = "ph_disabled";

  #[cfg(feature = "example")]
  pub const EXAMPLE: &str = "example";
}

pub mod logging;
pub use logging::Logger;

pub mod libargs;
pub mod structs;
pub mod ui;
pub mod execute;
pub mod sysinfo;

#[cfg(feature = "tlcl")]
pub mod tlcl;
