/// Common keys for the global key/value store.
pub mod keys {
  /// Internal disk path.
  pub const INTERNAL_DISK: &str = "internal_disk";

  /// Global file handle for the GSC device.
  pub const GSC_SOCKET: &str = "gsc_socket";

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

  /// Used for example-specific features.
  #[cfg(feature = "example")]
  pub const EXAMPLE: &str = "example";
}

use std::{
  collections::HashMap,
  fs::File,
  os::unix::net::UnixStream,
  sync::{Mutex, OnceLock},
};

pub mod erase;
pub mod get;
pub mod set;

pub use erase::kv_erase;
pub use get::kv_get;
pub use set::kv_set;

#[derive(Debug)]
pub enum KvValue {
  String(String),
  Int(i64),
  Bool(bool),
  Socket(UnixStream),
  File(File),
}

pub mod key_types {
  pub const STRING: &str = "string";
  pub const INT: &str = "int";
  pub const BOOL: &str = "bool";
  pub const SOCKET: &str = "socket";
  pub const FILE: &str = "file";
}

static KV: OnceLock<Mutex<HashMap<&'static str, KvValue>>> = OnceLock::new();

pub fn kv() -> &'static Mutex<HashMap<&'static str, KvValue>> {
  KV.get_or_init(|| Mutex::new(HashMap::new()))
}
