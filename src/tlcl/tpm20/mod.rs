#![allow(unused_assignments)] /* remove when vb2ex_tpm_send_recv is implemented */
#![allow(unused_mut)]

pub mod constants;
pub mod marshal;
pub mod types;
pub mod unmarshal;

use crate::{tlcl::vb2ex_tpm_send_recv, LOG_DBG};

pub fn tpm_get_response(
  command: constants::TPM_CC,
  command_body: *const core::ffi::c_void,
  response: constants::tpm2_response,
) -> u32 {
  let mut cr_buffer = [0u8; 2048];
  let mut out_size: i32 = 0;
  let mut res: u32 = 69420;
  let mut in_size: u32 = 0;

  out_size = marshal::tpm_marshal_command(command, command_body, &mut cr_buffer, 2048); // 2048 == sizeof(cr_buffer)
  if out_size < 0 {
    LOG_DBG!("command {:#}, failed to serialize", command);
    return crate::tlcl::constants::TPM_E_WRITE_FAILURE;
  }

  in_size = 2048; // 2048 == sizeof(cr_buffer)
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

  LOG_DBG!("command {:#}, return code {:#}", command, res);

  return crate::tlcl::constants::TPM_SUCCESS;
}

pub fn tpm_send_receive(
  command: constants::TPM_CC,
  command_body: *const core::ffi::c_void,
  response: constants::tpm2_response,
) -> u32 {
  tpm_get_response(command, command_body, response)
}

pub fn tpm_get_response_code(
  command: constants::TPM_CC,
  command_body: *const core::ffi::c_void,
) -> u32 {
  let mut response: constants::tpm2_response = unsafe { core::mem::zeroed() };
  tpm_send_receive(command, command_body, response)
}
