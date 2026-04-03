#![allow(non_upper_case_globals)]

use crate::tlcl::tpm20::constants::{
  tpm2_response, TPM2_Clear, TPM_CC, TPM_ST_NO_SESSIONS, TPM_ST_SESSIONS,
};

pub fn read_be16(src: *const u8) -> u16 {
  unsafe { ((*src.add(0) as u16) << 8) | ((*src.add(1) as u16) << 0) }
}

pub fn read_be32(src: *const u8) -> u32 {
  unsafe {
    ((*src.add(0) as u32) << 24)
      | ((*src.add(1) as u32) << 16)
      | ((*src.add(2) as u32) << 8)
      | ((*src.add(3) as u32) << 0)
  }
}

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

pub fn tpm_unmarshal_response(
  command: TPM_CC,
  response_body: *const core::ffi::c_void,
  mut cr_size: i32,
  mut response: tpm2_response,
) -> i32 {
  let mut buffer = response_body as *const u8;

  if cr_size < core::mem::size_of::<crate::tlcl::tpm20::constants::tpm_header>() as i32 {
    return -1;
  }

  response.hdr.tpm_tag = unmarshal_u16(&mut buffer, &mut cr_size);
  response.hdr.tpm_size = unmarshal_u32(&mut buffer, &mut cr_size);
  response.hdr.tpm_code = unmarshal_u32(&mut buffer, &mut cr_size);

  if response.hdr.tpm_tag != TPM_ST_NO_SESSIONS && response.hdr.tpm_tag != TPM_ST_SESSIONS {
    return -1;
  }

  if response.hdr.tpm_size
    < core::mem::size_of::<crate::tlcl::tpm20::constants::tpm_header>() as u32
  {
    return -1;
  }

  if !cr_size.eq(&0) {
    match command {
      TPM2_Clear => {
        cr_size = 0;
      }
      _ => {
        return -1;
      }
    }
  }

  if cr_size != 0 {
    return -1;
  }

  0
}
