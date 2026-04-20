#![allow(non_snake_case)]

use crate::tlcl::tpm12::{
  constants::{
    TPM_ORD_PhysicalEnable, TPM_PHYSICAL_PRESENCE_CMD_ENABLE, TPM_PHYSICAL_PRESENCE_PRESENT,
    TSC_ORD_PhysicalPresence, tpm1_physical_presence_cmd,
  },
  tpm_get_response_code,
};

pub fn TlclPhysicalPresenceCMDEnable() -> u32 {
  let command_body = tpm1_physical_presence_cmd {
    physical_presence: TPM_PHYSICAL_PRESENCE_CMD_ENABLE,
  };

  tpm_get_response_code(
    TSC_ORD_PhysicalPresence,
    &command_body as *const tpm1_physical_presence_cmd as *const core::ffi::c_void,
  )
}

pub fn TlclAssertPhysicalPresence() -> u32 {
  let command_body = tpm1_physical_presence_cmd {
    physical_presence: TPM_PHYSICAL_PRESENCE_PRESENT,
  };

  tpm_get_response_code(
    TSC_ORD_PhysicalPresence,
    &command_body as *const tpm1_physical_presence_cmd as *const core::ffi::c_void,
  )
}

pub fn TlclSetEnable() -> u32 {
  tpm_get_response_code(TPM_ORD_PhysicalEnable, core::ptr::null())
}
