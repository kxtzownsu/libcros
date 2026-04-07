#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]

use crate::tlcl::tpm12::constants::{
  TPM_BUFFER_SIZE, TPM_CMD_HEADER_SIZE, TPM_COMMAND, TPM_ORD_ForceClear, TPM_ORD_NV_DefineSpace,
  TPM_ORD_NV_ReadValue, TPM_ORD_NV_WriteValue, TPM_ORD_PhysicalEnable, TPM_ORD_SaveState,
  TPM_ORD_Startup, TPM_TAG_NV_ATTRIBUTES, TPM_TAG_NV_DATA_PUBLIC, TPM_TAG_RQU_COMMAND,
  TSC_ORD_PhysicalPresence, tpm_header, tpm1_nv_define_space_cmd, tpm1_nv_read_cmd,
  tpm1_nv_write_cmd, tpm1_physical_presence_cmd, tpm1_startup_cmd,
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

pub fn marshal_u8(buffer: &mut *mut u8, value: u8, buffer_space: &mut i32) {
  if *buffer_space < core::mem::size_of::<u8>() as i32 {
    *buffer_space = -1;
    return;
  }

  unsafe {
    **buffer = value;
    *buffer = (*buffer).add(core::mem::size_of::<u8>());
  }
  *buffer_space -= core::mem::size_of::<u8>() as i32;
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
    *buffer = (*buffer).add(blob_size);
  }
  *buffer_space -= blob_size as i32;
}

pub fn marshal_force_clear(_buffer: *mut u8, _buffer_space: &mut i32) {}

pub fn marshal_physical_enable(_buffer: *mut u8, _buffer_space: &mut i32) {}

pub fn marshal_physical_presence(
  mut buffer: *mut u8,
  command_body: *const tpm1_physical_presence_cmd,
  buffer_space: &mut i32,
) {
  if command_body.is_null() {
    *buffer_space = -1;
    return;
  }

  let command_body_ref = unsafe { &*command_body };
  marshal_u16(
    &mut buffer,
    command_body_ref.physical_presence,
    buffer_space,
  );
}

pub fn marshal_nv_read(
  mut buffer: *mut u8,
  command_body: *const tpm1_nv_read_cmd,
  buffer_space: &mut i32,
) {
  if command_body.is_null() {
    *buffer_space = -1;
    return;
  }

  let command_body_ref = unsafe { &*command_body };
  marshal_u32(&mut buffer, command_body_ref.nvIndex, buffer_space);
  marshal_u32(&mut buffer, command_body_ref.offset, buffer_space);
  marshal_u32(&mut buffer, command_body_ref.size, buffer_space);
}

pub fn marshal_nv_write(
  mut buffer: *mut u8,
  command_body: *const tpm1_nv_write_cmd,
  buffer_space: &mut i32,
) {
  if command_body.is_null() {
    *buffer_space = -1;
    return;
  }

  let command_body_ref = unsafe { &*command_body };
  marshal_u32(&mut buffer, command_body_ref.nvIndex, buffer_space);
  marshal_u32(&mut buffer, command_body_ref.offset, buffer_space);
  marshal_u32(&mut buffer, command_body_ref.size, buffer_space);
  marshal_blob(
    &mut buffer,
    command_body_ref.data,
    command_body_ref.size as usize,
    buffer_space,
  );
}

pub fn marshal_nv_define_space(
  mut buffer: *mut u8,
  command_body: *const tpm1_nv_define_space_cmd,
  buffer_space: &mut i32,
) {
  if command_body.is_null() {
    *buffer_space = -1;
    return;
  }

  let command_body_ref = unsafe { &*command_body };

  marshal_u16(&mut buffer, TPM_TAG_NV_DATA_PUBLIC, buffer_space);
  marshal_u32(&mut buffer, command_body_ref.nvIndex, buffer_space);

  if command_body_ref.auth_policy.is_null() {
    *buffer_space = -1;
    return;
  }

  marshal_blob(
    &mut buffer,
    command_body_ref.auth_policy as *const u8,
    core::mem::size_of::<crate::tlcl::tpm12::constants::TPM_NV_AUTH_POLICY>(),
    buffer_space,
  );

  marshal_u16(&mut buffer, TPM_TAG_NV_ATTRIBUTES, buffer_space);
  marshal_u32(&mut buffer, command_body_ref.perm, buffer_space);
  marshal_u8(&mut buffer, 0, buffer_space);
  marshal_u8(&mut buffer, 0, buffer_space);
  marshal_u8(&mut buffer, 0, buffer_space);
  marshal_u32(&mut buffer, command_body_ref.size, buffer_space);

  marshal_u32(&mut buffer, 0, buffer_space);
  marshal_u32(&mut buffer, 0, buffer_space);
  marshal_u32(&mut buffer, 0, buffer_space);
  marshal_u32(&mut buffer, 0, buffer_space);
  marshal_u32(&mut buffer, 0, buffer_space);
}

pub fn marshal_startup(
  mut buffer: *mut u8,
  command_body: *const tpm1_startup_cmd,
  buffer_space: &mut i32,
) {
  if command_body.is_null() {
    *buffer_space = -1;
    return;
  }

  let command_body_ref = unsafe { &*command_body };
  marshal_u16(&mut buffer, command_body_ref.startup_type, buffer_space);
}

pub fn marshal_savestate(_buffer: *mut u8, _buffer_space: &mut i32) {}

pub fn tpm_marshal_command(
  command: TPM_COMMAND,
  tpm_command_body: *const core::ffi::c_void,
  buffer: &mut [u8; TPM_BUFFER_SIZE],
  buffer_size: usize,
) -> i32 {
  let cmd_body =
    unsafe { (buffer.as_mut_ptr() as *mut u8).add(core::mem::size_of::<tpm_header>()) };
  let max_body_size: i32 = (buffer_size - core::mem::size_of::<tpm_header>())
    .try_into()
    .unwrap();
  let mut body_size = max_body_size;

  match command {
    TPM_ORD_ForceClear => {
      marshal_force_clear(cmd_body, &mut body_size);
    }
    TPM_ORD_PhysicalEnable => {
      marshal_physical_enable(cmd_body, &mut body_size);
    }
    TSC_ORD_PhysicalPresence => {
      marshal_physical_presence(
        cmd_body,
        tpm_command_body as *const tpm1_physical_presence_cmd,
        &mut body_size,
      );
    }
    TPM_ORD_NV_ReadValue => {
      marshal_nv_read(
        cmd_body,
        tpm_command_body as *const tpm1_nv_read_cmd,
        &mut body_size,
      );
    }
    TPM_ORD_NV_WriteValue => {
      marshal_nv_write(
        cmd_body,
        tpm_command_body as *const tpm1_nv_write_cmd,
        &mut body_size,
      );
    }
    TPM_ORD_NV_DefineSpace => {
      marshal_nv_define_space(
        cmd_body,
        tpm_command_body as *const tpm1_nv_define_space_cmd,
        &mut body_size,
      );
    }
    TPM_ORD_Startup => {
      marshal_startup(
        cmd_body,
        tpm_command_body as *const tpm1_startup_cmd,
        &mut body_size,
      );
    }
    TPM_ORD_SaveState => {
      marshal_savestate(cmd_body, &mut body_size);
    }
    _ => {
      body_size = -1;
    }
  }

  if body_size > 0 {
    let mut header_space = core::mem::size_of::<tpm_header>() as i32;
    let mut header = buffer.as_mut_ptr();

    body_size = max_body_size - body_size;
    body_size += TPM_CMD_HEADER_SIZE as i32;

    marshal_u16(&mut header, TPM_TAG_RQU_COMMAND, &mut header_space);
    marshal_u32(&mut header, body_size as u32, &mut header_space);
    marshal_u32(&mut header, command, &mut header_space);
  }

  body_size
}
