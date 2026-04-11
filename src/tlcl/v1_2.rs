pub struct Client;

impl Client {
  pub fn new() -> Self {
    Self
  }

  crate::tlcl::client::impl_tlcl_client_common!(tpm12);

  pub fn nv_read_public(&self, index: u32, presp: *mut core::ffi::c_void) -> u32 {
    crate::tlcl::commands::tpm12::TlclNVReadPublic(index, presp)
  }
}
