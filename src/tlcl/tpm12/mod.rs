pub mod constants;
pub mod marshal;
pub mod unmarshal;
pub mod utils;

pub fn tpm_get_response(
  command: constants::TPM_COMMAND,
  command_body: *const core::ffi::c_void,
  response: &mut constants::tpm1_response,
) -> u32 {
  crate::tlcl::backend::tpm_get_response::<{ constants::TPM_BUFFER_SIZE }, _, _, _, _>(
    command,
    command_body,
    response,
    marshal::tpm_marshal_command,
    unmarshal::tpm_unmarshal_response,
    |resp| unsafe { core::ptr::read_unaligned(core::ptr::addr_of!(resp.hdr.tpm_code)) },
  )
}

pub fn tpm_send_receive(
  command: constants::TPM_COMMAND,
  command_body: *const core::ffi::c_void,
  response: &mut constants::tpm1_response,
) -> u32 {
  let rv = tpm_get_response(command, command_body, response);

  if rv != crate::tlcl::constants::TPM_SUCCESS {
    return rv;
  }

  unsafe { core::ptr::read_unaligned(core::ptr::addr_of!(response.hdr.tpm_code)) }
}

pub fn tpm_get_response_code(
  command: constants::TPM_COMMAND,
  command_body: *const core::ffi::c_void,
) -> u32 {
  let mut response: constants::tpm1_response = unsafe { core::mem::zeroed() };
  tpm_send_receive(command, command_body, &mut response)
}
