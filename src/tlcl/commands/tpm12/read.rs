#![allow(non_snake_case)]

use crate::tlcl::{
  constants::{TPM_E_READ_EMPTY, TPM_E_RESPONSE_TOO_LARGE, TPM_SUCCESS},
  tpm12::{
    constants::{tpm1_nv_read_cmd, tpm1_response, TPM_ORD_NV_ReadValue},
    tpm_send_receive,
  },
};

pub fn TlclRead(index: u32, outbuf: *mut core::ffi::c_void, length: u32) -> u32 {
  TlclReadWithOffset(index, length, 0, outbuf)
}

pub fn TlclReadWithOffset(
  index: u32,
  length: u32,
  offset: u32,
  outbuf: *mut core::ffi::c_void,
) -> u32 {
  let nv_readc = tpm1_nv_read_cmd {
    nvIndex: index,
    offset,
    size: length,
  };
  let mut response: tpm1_response = unsafe { core::mem::zeroed() };
  let rv = tpm_send_receive(
    TPM_ORD_NV_ReadValue,
    &nv_readc as *const tpm1_nv_read_cmd as *const core::ffi::c_void,
    &mut response,
  );

  if rv != TPM_SUCCESS {
    return rv;
  }

  if length > response.nvr.data_size {
    return TPM_E_RESPONSE_TOO_LARGE;
  }

  if length < response.nvr.data_size {
    return TPM_E_READ_EMPTY;
  }

  unsafe {
    core::ptr::copy_nonoverlapping(
      response.nvr.data.as_ptr(),
      outbuf as *mut u8,
      length as usize,
    );
  }

  TPM_SUCCESS
}
