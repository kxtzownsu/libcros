#![allow(non_camel_case_types)]

pub type TPM_RC = u32;

pub const RC_FMT1: u32 = 0x080;
pub const RC_WARN: u32 = 0x900;

pub const TPM_RC_HANDLE: TPM_RC = RC_FMT1 + 0x00B;

pub const TPM_RC_RETRY: TPM_RC = RC_WARN + 0x023;
