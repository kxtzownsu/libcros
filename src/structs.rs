#[derive(Debug)]
pub struct TPM2B {
  pub size: u16,
  pub buffer: *const u8,
}

#[derive(Debug)]
pub struct Tpm2SessionHeader {
  pub session_handle: u32,
  pub nonce_size: u16,
  pub nonce: u8,
  pub session_attrs: u8,
  pub auth_size: u16,
  pub auth: u8,
}

#[derive(Debug)]
pub struct Tpm2TpmHeader {
  pub tpm_tag: u16,
  pub tpm_size: u32,
  pub tpm_code: u32,
}
