#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]

use crate::tlcl::{
  bytes::{read_be16, read_be32},
  tpm12::constants::{
    TPM_COMMAND, TPM_ORD_NV_ReadValue, TPM_RESP_HEADER_SIZE, TPM_SUCCESS, TPM_TAG_RSP_COMMAND,
    nv_read_response, tpm1_response,
  },
};

pub fn unmarshal_u16(buffer: &mut *const u8, buffer_space: &mut i32) -> u16 {
  let value: u16;

  if *buffer_space < core::mem::size_of::<u16>() as i32 {
    *buffer_space = -1;
    return 0;
  }

  value = read_be16(*buffer);
  unsafe {
    *buffer = (*buffer).add(core::mem::size_of::<u16>());
  }
  *buffer_space -= core::mem::size_of::<u16>() as i32;

  value
}

pub fn unmarshal_u32(buffer: &mut *const u8, buffer_space: &mut i32) -> u32 {
  let value: u32;

  if *buffer_space < core::mem::size_of::<u32>() as i32 {
    *buffer_space = -1;
    return 0;
  }

  value = read_be32(*buffer);
  unsafe {
    *buffer = (*buffer).add(core::mem::size_of::<u32>());
  }
  *buffer_space -= core::mem::size_of::<u32>() as i32;

  value
}

pub fn unmarshal_nv_read(buffer: &mut *const u8, size: &mut i32, nvr: &mut nv_read_response) {
  nvr.data_size = unmarshal_u32(buffer, size);
  if nvr.data_size as i32 > *size {
    *size = -1;
    nvr.data_size = 0;
    return;
  }

  if nvr.data_size as usize > nvr.data.len() {
    *size = -1;
    nvr.data_size = 0;
    return;
  }

  unsafe {
    core::ptr::copy_nonoverlapping(*buffer, nvr.data.as_mut_ptr(), nvr.data_size as usize);
    *buffer = (*buffer).add(nvr.data_size as usize);
  }
  *size -= nvr.data_size as i32;
}

pub fn tpm_unmarshal_response(
  command: TPM_COMMAND,
  response_body: *const core::ffi::c_void,
  mut cr_size: i32,
  response: &mut tpm1_response,
) -> i32 {
  let mut buffer = response_body as *const u8;

  if cr_size < TPM_RESP_HEADER_SIZE as i32 {
    return -1;
  }

  response.hdr.tpm_tag = unmarshal_u16(&mut buffer, &mut cr_size);
  response.hdr.tpm_size = unmarshal_u32(&mut buffer, &mut cr_size);
  response.hdr.tpm_code = unmarshal_u32(&mut buffer, &mut cr_size);

  if response.hdr.tpm_tag != TPM_TAG_RSP_COMMAND {
    return -1;
  }

  if response.hdr.tpm_size < TPM_RESP_HEADER_SIZE as u32 {
    return -1;
  }

  if response.hdr.tpm_code != TPM_SUCCESS {
    return 0;
  }

  if cr_size > 0 {
    match command {
      TPM_ORD_NV_ReadValue => {
        unmarshal_nv_read(&mut buffer, &mut cr_size, &mut response.nvr);
      }
      _ => {
        cr_size = 0;
      }
    }
  }

  if cr_size != 0 {
    return -1;
  }

  0
}
