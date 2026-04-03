#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use crate::{
  keys, kv_get, kv_set,
  tlcl::tpm20::constants::{
    tpm2_session_header, tpm_header, TPM2_Clear, TPM_RH_PLATFORM, TPM_RS_PW, TPM_ST_NO_SESSIONS,
    TPM_ST_SESSIONS,
  },
};

pub fn write_be16(dest: *mut u8, val: u16) {
  unsafe {
    *dest.add(0) = (val >> 8) as u8;
    *dest.add(1) = val as u8;
  }
}

pub fn write_be32(dest: *mut u8, val: u32) {
  unsafe {
    *dest.add(0) = (val >> 24) as u8;
    *dest.add(1) = (val >> 16) as u8;
    *dest.add(2) = (val >> 8) as u8;
    *dest.add(3) = val as u8;
  }
}

pub fn marshal_blob(
  buffer: &mut *mut u8,
  blob: *const u8,
  blob_size: usize,
  buffer_space: &mut i32,
) {
  if *buffer_space < blob_size as i32 {
    *buffer_space = -1;
    return;
  }

  unsafe {
    core::ptr::copy_nonoverlapping(blob, *buffer, blob_size);
    *buffer = ((*buffer as usize) + blob_size) as *mut u8;
  }
  *buffer_space -= blob_size as i32;
}

pub fn marshal_u8(buffer: &mut *mut u8, value: u8, buffer_space: &mut i32) {
  let mut bp = *buffer;

  if *buffer_space < core::mem::size_of::<u8>() as i32 {
    *buffer_space = -1;
    return;
  }

  unsafe {
    *bp = value;
    bp = bp.add(1);
  }
  *buffer = bp;
  *buffer_space -= core::mem::size_of::<u8>() as i32;
}

pub fn marshal_u16(buffer: &mut *mut u8, value: u16, buffer_space: &mut i32) {
  if *buffer_space < core::mem::size_of::<u16>() as i32 {
    *buffer_space = -1;
    return;
  }

  write_be16(*buffer, value);
  unsafe {
    *buffer = (*buffer).add(core::mem::size_of::<u16>());
  }
  *buffer_space -= core::mem::size_of::<u16>() as i32;
}

pub fn marshal_u32(buffer: &mut *mut u8, value: u32, buffer_space: &mut i32) {
  if *buffer_space < core::mem::size_of::<u32>() as i32 {
    *buffer_space = -1;
    return;
  }

  write_be32(*buffer, value);
  unsafe {
    *buffer = (*buffer).add(core::mem::size_of::<u32>());
  }
  *buffer_space -= core::mem::size_of::<u32>() as i32;
}

#[inline(always)]
pub fn marshal_TPM_HANDLE(buffer: &mut *mut u8, value: u32, buffer_space: &mut i32) {
  marshal_u32(buffer, value, buffer_space);
}

#[inline(always)]
pub fn marshal_TPM_SU(buffer: &mut *mut u8, value: u16, buffer_space: &mut i32) {
  marshal_u16(buffer, value, buffer_space);
}

#[inline(always)]
pub fn marshal_ALG_ID(buffer: &mut *mut u8, value: u16, buffer_space: &mut i32) {
  marshal_u16(buffer, value, buffer_space);
}

#[inline(always)]
pub fn marshal_TPMI_ALG_HASH(buffer: &mut *mut u8, value: u16, buffer_space: &mut i32) {
  marshal_u16(buffer, value, buffer_space);
}

pub struct tpm2_marshal_size_field {
  pub size: i32,
  pub location: *mut u8,
}

pub fn marshal_reserve_size_field(
  buffer: &mut *mut u8,
  field: &mut tpm2_marshal_size_field,
  field_size: i32,
  buffer_space: &mut i32,
) {
  if field_size != core::mem::size_of::<u16>() as i32
    && field_size != core::mem::size_of::<u32>() as i32
  {
    *buffer_space = -1;
    return;
  }
  if *buffer_space < field_size {
    *buffer_space = -1;
    return;
  }

  field.size = field_size;
  field.location = *buffer;
  *buffer_space -= field_size;
  *buffer = ((*buffer as usize) + field_size as usize) as *mut u8;
}

pub fn marshal_fill_size_field(
  buffer: &mut *mut u8,
  field: &mut tpm2_marshal_size_field,
  include_size_field: bool,
  buffer_space: &mut i32,
) {
  let mut size = (*buffer as usize) - (field.location as usize);

  if *buffer_space < 0 {
    return;
  }

  if !include_size_field {
    size -= field.size as usize;
  }

  if field.size == core::mem::size_of::<u32>() as i32 {
    marshal_u32(&mut field.location, size as u32, &mut field.size);
  } else {
    marshal_u16(&mut field.location, size as u16, &mut field.size);
  }
}

pub fn marshal_session_header(
  buffer: &mut *mut u8,
  session_header: &tpm2_session_header,
  buffer_space: &mut i32,
) {
  let mut size_field = tpm2_marshal_size_field {
    size: 0,
    location: core::ptr::null_mut(),
  };

  marshal_reserve_size_field(
    buffer,
    &mut size_field,
    core::mem::size_of::<u32>() as i32,
    buffer_space,
  );
  marshal_u32(buffer, session_header.session_handle, buffer_space);
  marshal_u16(buffer, session_header.nonce_size, buffer_space);
  marshal_blob(
    buffer,
    session_header.nonce as *const u8,
    session_header.nonce_size as usize,
    buffer_space,
  );
  let session_attrs = unsafe { session_header.attrs.session_attrs };
  marshal_u8(buffer, session_attrs, buffer_space);
  marshal_u16(buffer, session_header.auth_size, buffer_space);
  marshal_blob(
    buffer,
    session_header.auth as *const u8,
    session_header.auth_size as usize,
    buffer_space,
  );
  marshal_fill_size_field(buffer, &mut size_field, false, buffer_space);
}

pub fn marshal_clear(
  mut buffer: *mut u8,
  _command_body: *const core::ffi::c_void,
  buffer_space: &mut i32,
) {
  let mut session_header: tpm2_session_header;

  kv_set(keys::TPM_TAG, TPM_ST_SESSIONS);
  marshal_TPM_HANDLE(&mut buffer, TPM_RH_PLATFORM, buffer_space);
  unsafe {
    session_header = std::mem::zeroed();
  }
  session_header.session_handle = TPM_RS_PW;
  marshal_session_header(&mut buffer, &session_header, buffer_space);
}

pub fn tpm_marshal_command(
  command: crate::tlcl::tpm20::constants::TPM_CC,
  tpm_command_body: *const core::ffi::c_void,
  buffer: &mut [u8; 2048],
  buffer_size: usize,
) -> i32 {
  let cmd_body =
    unsafe { (buffer.as_mut_ptr() as *mut u8).add(core::mem::size_of::<tpm_header>()) };
  let max_body_size: i32 = (buffer_size - core::mem::size_of::<tpm_header>())
    .try_into()
    .unwrap();
  let mut body_size = max_body_size;

  kv_set(keys::TPM_TAG, TPM_ST_NO_SESSIONS);

  match command {
    TPM2_Clear => {
      marshal_clear(cmd_body, tpm_command_body, &mut body_size);
    }
    _ => {
      body_size = -1;
    }
  }

  if body_size > 0 {
    let mut header_space = core::mem::size_of::<tpm_header>() as i32;
    let mut header = buffer.as_mut_ptr();

    body_size = max_body_size - body_size;
    body_size += core::mem::size_of::<tpm_header>() as i32;

    marshal_u16(
      &mut header,
      kv_get(keys::TPM_TAG)
        .parse::<u16>()
        .unwrap_or(TPM_ST_NO_SESSIONS),
      &mut header_space,
    );
    marshal_u32(&mut header, body_size as u32, &mut header_space);
    marshal_u32(&mut header, command, &mut header_space);
  }

  body_size
}
