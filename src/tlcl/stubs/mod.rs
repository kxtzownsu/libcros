pub mod constants;
pub mod marshal;
pub mod unmarshal;

pub fn tpm_get_response(
  _command: u32,
  _command_body: *const core::ffi::c_void,
  _response: *mut core::ffi::c_void,
) -> u32 {
  crate::tlcl::constants::TPM_E_NO_SUCH_COMMAND
}

pub fn tpm_send_receive(
  _command: u32,
  _command_body: *const core::ffi::c_void,
  _response: *mut core::ffi::c_void,
) -> u32 {
  crate::tlcl::constants::TPM_E_NO_SUCH_COMMAND
}

pub fn tpm_get_response_code(_command: u32, _command_body: *const core::ffi::c_void) -> u32 {
  crate::tlcl::constants::TPM_E_NO_SUCH_COMMAND
}
