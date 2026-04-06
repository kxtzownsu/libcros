#![allow(non_snake_case)]

use crate::tlcl::{
  constants::{
    TPM_E_AUTHFAIL, TPM_E_BUFFER_SIZE, TPM_E_INVALID_RESPONSE, TPM_E_NO_SUCH_COMMAND,
    TPM_E_RESPONSE_TOO_LARGE, TPM_SUCCESS,
  },
  tpm12::{
    constants::{
      TPM_AUTH_DATA_LEN, TPM_BUFFER_SIZE, TPM_CAP_NV_INDEX, TPM_NV_AUTH_POLICY,
      TPM_ORD_GetCapability, TPM_ORD_NV_DefineSpace, TPM_TAG_RQU_COMMAND, TPM_TAG_RSP_COMMAND,
      tpm1_nv_define_space_cmd,
    },
    tpm_get_response_code,
    utils::{decode_pcr_info, read_be16, read_be32, write_be16, write_be32},
  },
  vb2ex_tpm_send_recv,
};

pub fn TlclDefineSpace(index: u32, perm: u32, size: u32) -> u32 {
  TlclDefineSpaceEx(
    core::ptr::null(),
    0,
    index,
    perm,
    size,
    core::ptr::null(),
    0,
  )
}

pub fn TlclUndefineSpace(index: u32) -> u32 {
  TlclUndefineSpaceEx(core::ptr::null(), 0, index)
}

pub fn TlclUndefineSpaceEx(owner_auth: *const u8, owner_auth_size: u32, index: u32) -> u32 {
  TlclDefineSpaceEx(
    owner_auth,
    owner_auth_size,
    index,
    0,
    0,
    core::ptr::null(),
    0,
  )
}

pub fn TlclDefineSpaceEx(
  owner_auth: *const u8,
  owner_auth_size: u32,
  index: u32,
  perm: u32,
  size: u32,
  auth_policy: *const core::ffi::c_void,
  auth_policy_size: u32,
) -> u32 {
  if !owner_auth.is_null() || owner_auth_size != 0 {
    if owner_auth_size as usize != TPM_AUTH_DATA_LEN {
      return TPM_E_AUTHFAIL;
    }
    return TPM_E_NO_SUCH_COMMAND;
  }

  if !auth_policy.is_null() && auth_policy_size != core::mem::size_of::<TPM_NV_AUTH_POLICY>() as u32
  {
    return TPM_E_BUFFER_SIZE;
  }

  let cmd = tpm1_nv_define_space_cmd {
    nvIndex: index,
    perm,
    size,
    auth_policy: auth_policy as *const TPM_NV_AUTH_POLICY,
  };

  tpm_get_response_code(
    TPM_ORD_NV_DefineSpace,
    &cmd as *const tpm1_nv_define_space_cmd as *const core::ffi::c_void,
  )
}

pub fn TlclInitNvAuthPolicy(
  _pcr_selection_bitmap: u32,
  _pcr_values: *const u8,
  auth_policy: *mut core::ffi::c_void,
  auth_policy_size: &mut u32,
) -> u32 {
  let required = core::mem::size_of::<TPM_NV_AUTH_POLICY>() as u32;
  let provided = *auth_policy_size;
  *auth_policy_size = required;

  if auth_policy.is_null() || provided < required {
    return TPM_E_BUFFER_SIZE;
  }

  unsafe {
    core::ptr::write_bytes(auth_policy as *mut u8, 0, required as usize);
  }

  TPM_SUCCESS
}

pub fn TlclGetSpaceInfo(
  index: u32,
  attributes: &mut u32,
  size: &mut u32,
  auth_policy: *mut core::ffi::c_void,
  auth_policy_size: &mut u32,
) -> u32 {
  let needed_policy_size = core::mem::size_of::<TPM_NV_AUTH_POLICY>() as u32;
  let provided_policy_size = *auth_policy_size;
  *auth_policy_size = needed_policy_size;
  if auth_policy.is_null() || provided_policy_size < needed_policy_size {
    return TPM_E_BUFFER_SIZE;
  }

  let mut cmd = [0u8; 22];
  write_be16(cmd.as_mut_ptr(), TPM_TAG_RQU_COMMAND);
  write_be32(unsafe { cmd.as_mut_ptr().add(2) }, cmd.len() as u32);
  write_be32(unsafe { cmd.as_mut_ptr().add(6) }, TPM_ORD_GetCapability);
  write_be32(unsafe { cmd.as_mut_ptr().add(10) }, TPM_CAP_NV_INDEX);
  write_be32(
    unsafe { cmd.as_mut_ptr().add(14) },
    core::mem::size_of::<u32>() as u32,
  );
  write_be32(unsafe { cmd.as_mut_ptr().add(18) }, index);

  let mut response = [0u8; TPM_BUFFER_SIZE];
  let mut response_size = TPM_BUFFER_SIZE as u32;
  let rv = vb2ex_tpm_send_recv(
    cmd.as_ptr(),
    cmd.len() as u32,
    response.as_mut_ptr(),
    &mut response_size,
  );
  if rv != TPM_SUCCESS {
    return rv;
  }

  if response_size as usize > response.len() {
    return TPM_E_RESPONSE_TOO_LARGE;
  }
  if response_size < 14 {
    return TPM_E_INVALID_RESPONSE;
  }

  if read_be16(response.as_ptr()) != TPM_TAG_RSP_COMMAND {
    return TPM_E_INVALID_RESPONSE;
  }

  let tpm_rc = read_be32(unsafe { response.as_ptr().add(6) });
  if tpm_rc != TPM_SUCCESS {
    return tpm_rc;
  }

  let mut cursor = 10usize;
  let cap_size = read_be32(unsafe { response.as_ptr().add(cursor) }) as usize;
  cursor += 4;
  if cap_size > response_size as usize - cursor {
    return TPM_E_INVALID_RESPONSE;
  }
  let end = cursor + cap_size;

  if end.saturating_sub(cursor) < 6 {
    return TPM_E_INVALID_RESPONSE;
  }
  cursor += 2;
  let response_index = read_be32(unsafe { response.as_ptr().add(cursor) });
  cursor += 4;
  if response_index != index {
    return TPM_E_INVALID_RESPONSE;
  }

  let policy = auth_policy as *mut TPM_NV_AUTH_POLICY;
  let pcr_read = unsafe { core::ptr::addr_of_mut!((*policy).pcr_info_read) };
  let pcr_write = unsafe { core::ptr::addr_of_mut!((*policy).pcr_info_write) };
  if !decode_pcr_info(&response, &mut cursor, end, pcr_read)
    || !decode_pcr_info(&response, &mut cursor, end, pcr_write)
  {
    return TPM_E_INVALID_RESPONSE;
  }

  if end.saturating_sub(cursor) != 13 {
    return TPM_E_INVALID_RESPONSE;
  }

  cursor += 2;
  *attributes = read_be32(unsafe { response.as_ptr().add(cursor) });
  cursor += 4;
  cursor += 3;
  *size = read_be32(unsafe { response.as_ptr().add(cursor) });

  TPM_SUCCESS
}

pub fn TlclGetPermissions(index: u32, permissions: &mut u32) -> u32 {
  let mut dummy_size: u32 = 0;
  let mut dummy_policy: TPM_NV_AUTH_POLICY = unsafe { core::mem::zeroed() };
  let mut dummy_policy_size: u32 = core::mem::size_of::<TPM_NV_AUTH_POLICY>() as u32;

  TlclGetSpaceInfo(
    index,
    permissions,
    &mut dummy_size,
    &mut dummy_policy as *mut TPM_NV_AUTH_POLICY as *mut core::ffi::c_void,
    &mut dummy_policy_size,
  )
}
