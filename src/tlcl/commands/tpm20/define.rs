#![allow(non_snake_case)]

use crate::tlcl::{
  commands::tpm20::TlclNVReadPublic,
  constants::{TPM_E_BUFFER_SIZE, TPM_SUCCESS},
  tpm20::{
    constants::{
      nv_read_public_response, tpm2_nv_define_space_cmd, tpm2_nv_undefine_space_cmd,
      TPM2_NV_DefineSpace, TPM2_NV_UndefineSpace, HR_NV_INDEX, TPMA_NV_AUTHREAD, TPMA_NV_AUTHWRITE,
      TPMA_NV_MASK_READ, TPMA_NV_MASK_WRITE, TPMA_NV_PLATFORMCREATE, TPM_ALG_SHA256,
    },
    tpm_get_response_code,
  },
};

pub fn TlclGetPermissions(index: u32, permissions: &mut u32) -> u32 {
  let mut pub_resp: nv_read_public_response = unsafe { core::mem::zeroed() };
  let rv = TlclNVReadPublic(index, &mut pub_resp);
  if rv != TPM_SUCCESS {
    return rv;
  }

  *permissions =
    unsafe { core::ptr::read_unaligned(core::ptr::addr_of!(pub_resp.nvPublic.attributes)) };
  TPM_SUCCESS
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
  let mut define_space: tpm2_nv_define_space_cmd = unsafe { core::mem::zeroed() };
  let mut perm = perm;

  if (perm & TPMA_NV_MASK_WRITE) == 0 {
    perm |= TPMA_NV_AUTHWRITE;
  }
  if (perm & TPMA_NV_MASK_READ) == 0 {
    perm |= TPMA_NV_AUTHREAD;
  }

  define_space.publicInfo.nvIndex = HR_NV_INDEX + index;
  define_space.publicInfo.dataSize = size as u16;
  define_space.publicInfo.attributes = perm;
  define_space.publicInfo.nameAlg = TPM_ALG_SHA256;

  if !owner_auth.is_null() && owner_auth_size > 0 {
    define_space.auth.size = owner_auth_size as u16;
    define_space.auth.buffer = owner_auth;
  }

  if !auth_policy.is_null() && auth_policy_size > 0 {
    define_space.publicInfo.authPolicy.size = auth_policy_size as u16;
    define_space.publicInfo.authPolicy.buffer = auth_policy as *const u8;
  }

  tpm_get_response_code(
    TPM2_NV_DefineSpace,
    &define_space as *const tpm2_nv_define_space_cmd as *const core::ffi::c_void,
  )
}

pub fn TlclUndefineSpaceEx(_owner_auth: *const u8, _owner_auth_size: u32, index: u32) -> u32 {
  let mut undefine_space: tpm2_nv_undefine_space_cmd = unsafe { core::mem::zeroed() };
  let mut permissions: u32 = 0;
  let rv: u32;

  rv = TlclGetPermissions(index, &mut permissions);
  if rv != TPM_SUCCESS {
    return rv;
  }
  undefine_space.nvIndex = HR_NV_INDEX + index;
  undefine_space.use_platform_auth = ((permissions & TPMA_NV_PLATFORMCREATE) > 0) as u8;

  tpm_get_response_code(
    TPM2_NV_UndefineSpace,
    &undefine_space as *const tpm2_nv_undefine_space_cmd as *const core::ffi::c_void,
  )
}

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

pub fn TlclInitNvAuthPolicy(
  _pcr_selection_bitmap: u32,
  _pcr_values: *const u8,
  _auth_policy: *mut core::ffi::c_void,
  auth_policy_size: &mut u32,
) -> u32 {
  *auth_policy_size = 0;
  TPM_SUCCESS
}

pub fn TlclGetSpaceInfo(
  index: u32,
  attributes: &mut u32,
  size: &mut u32,
  auth_policy: *mut core::ffi::c_void,
  auth_policy_size: &mut u32,
) -> u32 {
  let mut resp: nv_read_public_response = unsafe { core::mem::zeroed() };
  let rv = TlclNVReadPublic(index, &mut resp);
  if rv != TPM_SUCCESS {
    return rv;
  }

  *attributes = unsafe { core::ptr::read_unaligned(core::ptr::addr_of!(resp.nvPublic.attributes)) };
  *size = unsafe { core::ptr::read_unaligned(core::ptr::addr_of!(resp.nvPublic.dataSize)) } as u32;

  let policy_size =
    unsafe { core::ptr::read_unaligned(core::ptr::addr_of!(resp.nvPublic.authPolicy.size)) } as u32;
  if policy_size > *auth_policy_size {
    return TPM_E_BUFFER_SIZE;
  }
  if policy_size > 0 && auth_policy.is_null() {
    return TPM_E_BUFFER_SIZE;
  }

  *auth_policy_size = policy_size;
  if policy_size > 0 {
    unsafe {
      core::ptr::copy_nonoverlapping(
        resp.nvPublic.authPolicy.buffer,
        auth_policy as *mut u8,
        policy_size as usize,
      );
    }
  }

  TPM_SUCCESS
}
