#![allow(non_camel_case_types)]

pub type TPM_COMMAND = u32;
pub type TPM_CC = u32;

#[derive(Copy, Clone)]
pub struct tpm_header {
  pub tpm_code: u32,
}

#[derive(Copy, Clone)]
pub struct tpm1_response {
  pub hdr: tpm_header,
}

#[derive(Copy, Clone)]
pub struct tpm2_response {
  pub hdr: tpm_header,
}

pub const TPM_BUFFER_SIZE: usize = 2048;
