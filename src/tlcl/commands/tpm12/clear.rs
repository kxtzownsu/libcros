#![allow(non_snake_case)]

use crate::tlcl::tpm12::{constants::TPM_ORD_ForceClear, tpm_get_response_code};

pub fn TlclForceClear() -> u32 {
  tpm_get_response_code(TPM_ORD_ForceClear, core::ptr::null())
}
