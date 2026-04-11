use crate::tlcl::tpm20::constants::nv_read_public_response;

pub struct Client;

impl Client {
  pub fn new() -> Self {
    Self
  }

  crate::tlcl::client::impl_tlcl_client_common!(tpm20);

  pub fn nv_read_public(&self, index: u32, presp: *mut nv_read_public_response) -> u32 {
    crate::tlcl::commands::tpm20::TlclNVReadPublic(index, presp as *mut core::ffi::c_void)
  }
}
