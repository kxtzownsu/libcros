#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]

use crate::{
  LOG_DBG,
  tlcl::tpm20::constants::{
    TPM_CC, TPM_ST_NO_SESSIONS, TPM_ST_SESSIONS, TPM2_Clear, TPM2_NV_DefineSpace, TPM2_NV_Read,
    TPM2_NV_ReadPublic, TPM2_NV_UndefineSpace, TPM2_NV_Write, TPM2B, TPMS_NV_PUBLIC,
    nv_read_public_response, nv_read_response, tpm2_response,
  },
};

pub fn unmarshal_u8(buffer: &mut *const u8, buffer_space: &mut i32) -> u8 {
  let value: u8;

  if *buffer_space < core::mem::size_of::<u8>() as i32 {
    *buffer_space = -1;
    return 0;
  }

  unsafe {
    value = **buffer;
    *buffer = (*buffer).add(core::mem::size_of::<u8>());
  }
  *buffer_space -= core::mem::size_of::<u8>() as i32;

  value
}

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

pub fn unmarshal_TPM2B(buffer: &mut *const u8, size: &mut i32, tpm2b: &mut TPM2B) {
  tpm2b.size = unmarshal_u16(buffer, size);
  if tpm2b.size as i32 > *size {
    tpm2b.buffer = core::ptr::null();
    tpm2b.size = 0;
    *size = -1;
    return;
  }

  tpm2b.buffer = *buffer;
  unsafe {
    *buffer = (*buffer).add(tpm2b.size as usize);
  }
  *size -= tpm2b.size as i32;
}

pub fn unmarshal_authorization_section(buffer: &mut *const u8, size: &mut i32, cmd_name: &str) {
  if *size != 5 {
    LOG_DBG!(
      "unexepcted authorization section size {} for {}",
      *size,
      &cmd_name
    );
  }
  *buffer = ((*buffer as usize) + (*size as usize)) as *const u8;
  *size = 0;
}

pub fn unmarshal_nv_read(buffer: &mut *const u8, size: &mut i32, nvr: &mut nv_read_response) {
  nvr.params_size = unmarshal_u32(buffer, size);
  unmarshal_TPM2B(buffer, size, &mut nvr.buffer);

  if nvr.params_size != (nvr.buffer.size as u32 + core::mem::size_of::<u16>() as u32) {
    return;
  }

  if *size < 0 {
    return;
  }

  unmarshal_authorization_section(buffer, size, "NV_Read");
}

pub fn unmarshal_nv_write(buffer: &mut *const u8, size: &mut i32) {
  unmarshal_authorization_section(buffer, size, "NV_Write");
}

pub fn unmarshal_TPMS_NV_PUBLIC(
  buffer: &mut *const u8,
  size: &mut i32,
  pub_data: &mut TPMS_NV_PUBLIC,
) {
  let mut tpm2b_size = unmarshal_u16(buffer, size) as i32;
  if tpm2b_size > *size {
    *size = -1;
    return;
  }
  *size -= tpm2b_size;

  pub_data.nvIndex = unmarshal_u32(buffer, &mut tpm2b_size);
  pub_data.nameAlg = unmarshal_u16(buffer, &mut tpm2b_size);
  pub_data.attributes = unmarshal_u32(buffer, &mut tpm2b_size);
  unmarshal_TPM2B(buffer, &mut tpm2b_size, &mut pub_data.authPolicy);
  pub_data.dataSize = unmarshal_u16(buffer, &mut tpm2b_size);

  if tpm2b_size != 0 {
    *size = -1;
  }
}

pub fn unmarshal_nv_read_public(
  buffer: &mut *const u8,
  size: &mut i32,
  nv_pub: &mut nv_read_public_response,
) {
  let mut nv_public: TPMS_NV_PUBLIC = unsafe { core::mem::zeroed() };
  let mut nv_name: TPM2B = unsafe { core::mem::zeroed() };

  unmarshal_TPMS_NV_PUBLIC(buffer, size, &mut nv_public);
  unmarshal_TPM2B(buffer, size, &mut nv_name);

  if *size < 0 {
    return;
  }

  if *size > 0 {
    *size = -1;
    return;
  }

  unsafe {
    core::ptr::addr_of_mut!(nv_pub.nvPublic).write_unaligned(nv_public);
    core::ptr::addr_of_mut!(nv_pub.nvName).write_unaligned(nv_name);
  }
}

pub fn tpm_unmarshal_response(
  command: TPM_CC,
  response_body: *const core::ffi::c_void,
  mut cr_size: i32,
  response: &mut tpm2_response,
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

  if cr_size == 0 {
    return 0;
  }

  match command {
    TPM2_NV_Read => {
      let mut nvr = nv_read_response {
        params_size: 0,
        buffer: TPM2B {
          size: 0,
          buffer: core::ptr::null(),
        },
      };
      unmarshal_nv_read(&mut buffer, &mut cr_size, &mut nvr);
      response.body.nvr = core::mem::ManuallyDrop::new(nvr);
    }
    TPM2_NV_ReadPublic => {
      let mut nv_pub: nv_read_public_response = unsafe { core::mem::zeroed() };
      unmarshal_nv_read_public(&mut buffer, &mut cr_size, &mut nv_pub);
      response.body.nv_read_public = core::mem::ManuallyDrop::new(nv_pub);
    }
    TPM2_NV_Write => {
      cr_size = 0;
    }
    TPM2_NV_DefineSpace => {
      cr_size = 0;
    }
    TPM2_NV_UndefineSpace => {
      cr_size = 0;
    }
    TPM2_Clear => {
      cr_size = 0;
    }
    _ => {
      return -1;
    }
  }

  if cr_size != 0 {
    return -1;
  }

  0
}
