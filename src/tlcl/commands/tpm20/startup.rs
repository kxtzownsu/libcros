#![allow(non_snake_case)]

use crate::tlcl::tpm20::{
  constants::{
    TPM_SU_CLEAR, TPM_SU_STATE, TPM2_Shutdown, TPM2_Startup, tpm2_shutdown_cmd, tpm2_startup_cmd,
  },
  tpm_get_response_code,
};

pub fn TlclStartup() -> u32 {
  let startup = tpm2_startup_cmd {
    startup_type: TPM_SU_CLEAR,
  };

  return tpm_get_response_code(
    TPM2_Startup,
    &startup as *const tpm2_startup_cmd as *const core::ffi::c_void,
  );
}

pub fn TlclSaveState() -> u32 {
  let shutdown = tpm2_shutdown_cmd {
    shutdown_type: TPM_SU_STATE,
  };

  return tpm_get_response_code(
    TPM2_Shutdown,
    &shutdown as *const tpm2_shutdown_cmd as *const core::ffi::c_void,
  );
}

pub fn TlclResume() -> u32 {
  let startup = tpm2_startup_cmd {
    startup_type: TPM_SU_STATE,
  };

  return tpm_get_response_code(
    TPM2_Startup,
    &startup as *const tpm2_startup_cmd as *const core::ffi::c_void,
  );
}
