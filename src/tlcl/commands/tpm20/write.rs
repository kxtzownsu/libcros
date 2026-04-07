#![allow(non_snake_case)]

use crate::tlcl::tpm20::{
  constants::{tpm2_nv_write_cmd, TPM2_NV_Write, HR_NV_INDEX, TPM2B},
  tpm_get_response_code,
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
  let nv_writeData = TPM2B {
    size: length as u16,
    buffer: data as *const u8,
  };

  let nv_writec = tpm2_nv_write_cmd {
    nvIndex: HR_NV_INDEX + index,
    data: nv_writeData,
    offset: offset.try_into().unwrap(),
  };

  return tpm_get_response_code(
    TPM2_NV_Write,
    &nv_writec as *const tpm2_nv_write_cmd as *const core::ffi::c_void,
  );
}
