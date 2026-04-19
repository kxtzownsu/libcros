#![allow(unused_imports)]
use crate::LOG_DBG;

pub mod backend;
pub mod stubs;
pub mod rollback;
// pub mod gsc;

pub use stubs::*;
pub use rollback::*;
// pub use gsc::*;

/* For functions that can't be categorized */

/// Returns the version of TPM that Tlcl was compiled with, e.g:
/// if feature `tpm 2_0` is enabled, then this will return "TPM 2.0".
#[cfg(feature = "tlcl")]
pub fn get_tpm_version() -> String {
  crate::tlcl::TlclGetTPMVersion()
}

/// This is a stub for whenever Tlcl isn't enabled. Due to the fact that we
/// can't fetch the TPM version without the Tlcl library, we just stub it here.
#[cfg(not(feature = "tlcl"))]
pub fn get_tpm_version() -> String {
  LOG_DBG!("tlcl feature not enabled");
  "Tlcl was not enabled when compiling libcros.".to_string()
}