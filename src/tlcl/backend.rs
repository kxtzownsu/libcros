use crate::{tlcl::vb2ex_tpm_send_recv, LOG_DBG};

pub(crate) fn tpm_get_response<const N: usize, R, MarshalFn, UnmarshalFn, CodeFn>(
  command: u32,
  command_body: *const core::ffi::c_void,
  response: &mut R,
  marshal_fn: MarshalFn,
  unmarshal_fn: UnmarshalFn,
  code_fn: CodeFn,
) -> u32
where
  MarshalFn: Fn(u32, *const core::ffi::c_void, &mut [u8; N], usize) -> i32,
  UnmarshalFn: Fn(u32, *const core::ffi::c_void, i32, &mut R) -> i32,
  CodeFn: Fn(&R) -> u32,
{
  let mut cr_buffer = [0u8; N];

  let out_size = marshal_fn(command, command_body, &mut cr_buffer, N);
  if out_size < 0 {
    LOG_DBG!("command 0x{:X}, failed to serialize", command);
    return crate::tlcl::constants::TPM_E_WRITE_FAILURE;
  }

  let mut in_size = N as u32;
  let res = vb2ex_tpm_send_recv(
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

  if unmarshal_fn(
    command,
    cr_buffer.as_ptr() as *const core::ffi::c_void,
    in_size as i32,
    response,
  ) < 0
  {
    LOG_DBG!("command 0x{:X}, failed to parse response", command);
    return crate::tlcl::constants::TPM_E_READ_FAILURE;
  }

  LOG_DBG!(
    "command 0x{:X}, return code 0x{:X}",
    command,
    code_fn(response)
  );
  crate::tlcl::constants::TPM_SUCCESS
}
