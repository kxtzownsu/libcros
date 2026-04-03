#![allow(non_snake_case)]

use crate::tlcl::tpm20::{constants::TPM2_Clear, tpm_get_response_code};

pub fn TlclForceClear() -> u32 {
  tpm_get_response_code(TPM2_Clear, std::ptr::null())
}
