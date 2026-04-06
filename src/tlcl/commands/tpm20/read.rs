#![allow(non_snake_case)]

use crate::tlcl::{
  constants::{TPM_E_BADINDEX, TPM_E_READ_EMPTY, TPM_E_RESPONSE_TOO_LARGE, TPM_SUCCESS},
  tpm20::{
    constants::{
      HR_NV_INDEX, TPM2_NV_Read, TPM2_NV_ReadPublic, nv_read_public_response, tpm2_nv_read_cmd,
      tpm2_nv_read_public_cmd, tpm2_response,
    },
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
  let mut nv_readc: tpm2_nv_read_cmd = unsafe { core::mem::zeroed() };
  let mut response: tpm2_response = unsafe { core::mem::zeroed() };
  let rv: u32;

  nv_readc.nvIndex = HR_NV_INDEX + index;
  nv_readc.size = length as u16;
  nv_readc.offset = offset as u16;

  rv = tpm_send_receive(
    TPM2_NV_Read,
    &nv_readc as *const tpm2_nv_read_cmd as *const core::ffi::c_void,
    &mut response,
  );

  match rv {
    TPM_SUCCESS => {}
    0x14a | 0x28b => {
      return TPM_E_BADINDEX;
    }
    _ => {
      return rv;
    }
  }

  let nvr = unsafe { &response.body.nvr };

  if length > nvr.buffer.size as u32 {
    return TPM_E_RESPONSE_TOO_LARGE;
  }

  if length < nvr.buffer.size as u32 {
    return TPM_E_READ_EMPTY;
  }

  unsafe {
    core::ptr::copy_nonoverlapping(nvr.buffer.buffer, outbuf as *mut u8, length as usize);
  }

  TPM_SUCCESS
}

pub fn TlclNVReadPublic(index: u32, presp: *mut nv_read_public_response) -> u32 {
  let mut response: tpm2_response = unsafe { core::mem::zeroed() };
  let mut read_pub: tpm2_nv_read_public_cmd = unsafe { core::mem::zeroed() };
  let rv: u32;

  read_pub.nvIndex = HR_NV_INDEX + index;

  rv = tpm_send_receive(
    TPM2_NV_ReadPublic,
    &read_pub as *const tpm2_nv_read_public_cmd as *const core::ffi::c_void,
    &mut response,
  );
  if rv == TPM_SUCCESS {
    unsafe {
      core::ptr::copy_nonoverlapping(
        &response.body.nv_read_public as *const core::mem::ManuallyDrop<nv_read_public_response>
          as *const nv_read_public_response,
        presp,
        1,
      );
    }
  }

  rv
}
