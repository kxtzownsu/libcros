pub mod constants;
pub mod marshal;
pub mod unmarshal;

use crate::{LOG_DBG, tlcl::vb2ex_tpm_send_recv};

pub fn tpm_get_response(
  command: constants::TPM_COMMAND,
  command_body: *const core::ffi::c_void,
  response: &mut constants::tpm1_response,
) -> u32 {
  let mut cr_buffer = [0u8; constants::TPM_BUFFER_SIZE];
  let out_size: i32;
  let res: u32;
  let mut in_size: u32 = constants::TPM_BUFFER_SIZE as u32;

  out_size = marshal::tpm_marshal_command(
    command,
    command_body,
    &mut cr_buffer,
    constants::TPM_BUFFER_SIZE,
  );
  if out_size < 0 {
    LOG_DBG!("command {:#}, failed to serialize", command);
    return crate::tlcl::constants::TPM_E_WRITE_FAILURE;
  }

  res = vb2ex_tpm_send_recv(
    cr_buffer.as_ptr(),
    out_size as u32,
    cr_buffer.as_mut_ptr(),
    &mut in_size,
  );
  if res != crate::tlcl::constants::TPM_SUCCESS {
    LOG_DBG!(
      "tpm transaction failed for {:#} with error code {:#}",
      command,
      res
    );
    return res;
  }

  if unmarshal::tpm_unmarshal_response(
    command,
    cr_buffer.as_ptr() as *const core::ffi::c_void,
    in_size as i32,
    response,
  ) < 0
  {
    LOG_DBG!("command {:#}, failed to parse response", command);
    return crate::tlcl::constants::TPM_E_READ_FAILURE;
  }

  let tpm_code = unsafe { core::ptr::read_unaligned(core::ptr::addr_of!(response.hdr.tpm_code)) };
  LOG_DBG!("command 0x{:X}, return code 0x{:X}", command, tpm_code);

  crate::tlcl::constants::TPM_SUCCESS
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
