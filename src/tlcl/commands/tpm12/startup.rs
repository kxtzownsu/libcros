#![allow(non_snake_case)]

use crate::tlcl::tpm12::{
  constants::{TPM_ORD_SaveState, TPM_ORD_Startup, TPM_ST_CLEAR, TPM_ST_STATE, tpm1_startup_cmd},
  tpm_get_response_code,
};

pub fn TlclStartup() -> u32 {
  let startup = tpm1_startup_cmd {
    startup_type: TPM_ST_CLEAR,
  };

  tpm_get_response_code(
    TPM_ORD_Startup,
    &startup as *const tpm1_startup_cmd as *const core::ffi::c_void,
  )
}

pub fn TlclSaveState() -> u32 {
  tpm_get_response_code(TPM_ORD_SaveState, core::ptr::null())
}

pub fn TlclResume() -> u32 {
  let startup = tpm1_startup_cmd {
    startup_type: TPM_ST_STATE,
  };

  tpm_get_response_code(
    TPM_ORD_Startup,
    &startup as *const tpm1_startup_cmd as *const core::ffi::c_void,
  )
}
