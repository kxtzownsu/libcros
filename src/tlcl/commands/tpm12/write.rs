#![allow(non_snake_case)]

use crate::tlcl::{
  tpm12::{
    constants::{TPM_ORD_NV_WriteValue, tpm1_nv_write_cmd},
    tpm_get_response_code,
  },
};

pub fn TlclWrite(index: u32, data: *const core::ffi::c_void, length: u32) -> u32 {
  TlclWriteWithOffset(index, data, length, 0)
}

pub fn TlclWriteWithOffset(
  index: u32,
  data: *const core::ffi::c_void,
  length: u32,
  offset: u32,
) -> u32 {
  let nv_writec = tpm1_nv_write_cmd {
    nvIndex: index,
    offset,
    size: length,
    data: data as *const u8,
  };

  return tpm_get_response_code(
    TPM_ORD_NV_WriteValue,
    &nv_writec as *const tpm1_nv_write_cmd as *const core::ffi::c_void,
  );
}
