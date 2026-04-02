#![allow(non_snake_case)]

use crate::{
  LOG_DBG,
  structs::{Tpm2SessionHeader, Tpm2TpmHeader},
  tlcl::{
    commands::tpm20::read::TlclGetPermissions,
    marshal::{marshal_session_header, marshal_tpm_header, marshal_u16, marshal_u32},
    tpm_exchange,
    tpm20::constants::{
      HR_NV_INDEX, TPM_ALG_SHA256, TPM_CMD_HEADER_SIZE, TPM_RH_OWNER, TPM_RH_PLATFORM, TPM_RS_PW,
      TPM_ST_SESSIONS, TPM2_NV_DefineSpace, TPM2_NV_UndefineSpace, TPMA_NV_AUTHREAD,
      TPMA_NV_AUTHWRITE, TPMA_NV_MASK_READ, TPMA_NV_MASK_WRITE, TPMA_NV_PLATFORMCREATE,
      TPMA_NV_PPREAD, TPMA_NV_PPWRITE,
    },
    unmarshal::unmarshal_response_code,
  },
};

fn define_space_auth(perm: u32) -> u32 {
  if perm & TPMA_NV_PLATFORMCREATE != 0 {
    TPM_RH_PLATFORM
  } else {
    TPM_RH_OWNER
  }
}

fn serialize_nv_define_space(
  nv_index: u32,
  perm: u32,
  size: u16,
  auth_value: &[u8],
  buf: &mut [u8; 512],
) {
  let mut offset = TPM_CMD_HEADER_SIZE;
  let mut max_size = 512 - TPM_CMD_HEADER_SIZE;
  let mut remaining = max_size;

  marshal_u32(define_space_auth(perm), &mut offset, buf, &mut remaining);

  marshal_session_header(
    Tpm2SessionHeader {
      session_handle: TPM_RS_PW,
      nonce_size: 0,
      nonce: 0,
      session_attrs: 0,
      auth_size: 0,
      auth: 0,
    },
    buf,
    &mut remaining,
    &mut offset,
  );

  /* TPM2B auth value (empty) */
  marshal_u16(auth_value.len() as u16, &mut offset, buf, &mut remaining);
  if !auth_value.is_empty() {
    buf[offset..offset + auth_value.len()].copy_from_slice(auth_value);
    offset += auth_value.len();
    remaining -= auth_value.len();
  }

  /* TPMS_NV_PUBLIC wrapped in TPM2B (size-prefixed) */
  let nv_public_size: u16 = 4 + 2 + 4 + 2 + 2; /* nvIndex + nameAlg + attributes + authPolicy(size only) + dataSize */
  marshal_u16(nv_public_size, &mut offset, buf, &mut remaining);
  marshal_u32(nv_index, &mut offset, buf, &mut remaining); /* nvIndex */
  marshal_u16(TPM_ALG_SHA256, &mut offset, buf, &mut remaining); /* nameAlg */
  marshal_u32(perm, &mut offset, buf, &mut remaining); /* attributes */
  marshal_u16(0, &mut offset, buf, &mut remaining); /* authPolicy size = 0 */
  marshal_u16(size, &mut offset, buf, &mut remaining); /* dataSize */

  let total_size = (max_size - remaining + TPM_CMD_HEADER_SIZE) as u32;
  marshal_tpm_header(
    Tpm2TpmHeader {
      tpm_tag: TPM_ST_SESSIONS,
      tpm_size: total_size,
      tpm_code: TPM2_NV_DefineSpace,
    },
    buf,
    &mut max_size,
    &mut 0,
  );
}

fn serialize_nv_undefine_space(nv_index: u32, use_platform_auth: bool, buf: &mut [u8; 512]) {
  let mut offset = TPM_CMD_HEADER_SIZE;
  let mut max_size = 512 - TPM_CMD_HEADER_SIZE;
  let mut remaining = max_size;

  let auth_handle = if use_platform_auth {
    TPM_RH_PLATFORM
  } else {
    TPM_RH_OWNER
  };
  marshal_u32(auth_handle, &mut offset, buf, &mut remaining);
  marshal_u32(nv_index, &mut offset, buf, &mut remaining);

  marshal_session_header(
    Tpm2SessionHeader {
      session_handle: TPM_RS_PW,
      nonce_size: 0,
      nonce: 0,
      session_attrs: 0,
      auth_size: 0,
      auth: 0,
    },
    buf,
    &mut remaining,
    &mut offset,
  );

  let total_size = (max_size - remaining + TPM_CMD_HEADER_SIZE) as u32;
  marshal_tpm_header(
    Tpm2TpmHeader {
      tpm_tag: TPM_ST_SESSIONS,
      tpm_size: total_size,
      tpm_code: TPM2_NV_UndefineSpace,
    },
    buf,
    &mut max_size,
    &mut 0,
  );
}

pub fn TlclDefineSpace(index: u32, perm: u32, size: u32) -> u32 {
  TlclDefineSpaceEx(None, index, perm, size, None)
}

pub fn TlclDefineSpaceEx(
  _owner_auth: Option<&[u8]>, /* auth not yet implemented */
  index: u32,
  mut perm: u32,
  size: u32,
  auth_policy: Option<&[u8]>,
) -> u32 {
  if perm & TPMA_NV_PLATFORMCREATE == 0 {
    /*  Owner space defaults */
    if perm & TPMA_NV_MASK_WRITE == 0 {
      perm |= TPMA_NV_AUTHWRITE;
    }
    if perm & TPMA_NV_MASK_READ == 0 {
      perm |= TPMA_NV_AUTHREAD;
    }
  } else {
    /* Platform space defaults */
    if perm & TPMA_NV_MASK_WRITE == 0 {
      perm |= TPMA_NV_PPWRITE;
    }
    if perm & TPMA_NV_MASK_READ == 0 {
      perm |= TPMA_NV_PPREAD;
    }
  }

  if let Some(policy) = auth_policy {
    if !policy.is_empty() {
      LOG_DBG!("auth_policy not implemented, ignoring");
    }
  }

  let nv_index = HR_NV_INDEX + index;
  let mut cmd_buf: [u8; 512] = [0; 512];
  let mut resp_buf: [u8; 4096] = [0; 4096];

  serialize_nv_define_space(nv_index, perm, size as u16, &[], &mut cmd_buf);
  LOG_DBG!("index=0x{:x}, perm=0x{:x}, size={}", index, perm, size);

  match tpm_exchange(&mut cmd_buf, &mut resp_buf) {
    Ok(_) => {
      let rc = unmarshal_response_code(&resp_buf);
      LOG_DBG!("rc=0x{:x}", rc);
      rc
    }
    Err(e) => {
      LOG_DBG!("tpm_exchange failed: {}", e);
      0xFFFFFFFF
    }
  }
}

pub fn TlclUndefineSpace(index: u32) -> u32 {
  TlclUndefineSpaceEx(None, index)
}

pub fn TlclUndefineSpaceEx(_owner_auth: Option<&[u8]>, index: u32) -> u32 {
  let nv_index = HR_NV_INDEX + index;

  let permissions = TlclGetPermissions(index);
  let use_platform_auth = permissions & TPMA_NV_PLATFORMCREATE != 0;

  let mut cmd_buf: [u8; 512] = [0; 512];
  let mut resp_buf: [u8; 4096] = [0; 4096];

  serialize_nv_undefine_space(nv_index, use_platform_auth, &mut cmd_buf);
  LOG_DBG!("index=0x{:x}, platform_auth={}", index, use_platform_auth);

  match tpm_exchange(&mut cmd_buf, &mut resp_buf) {
    Ok(_) => {
      let rc = unmarshal_response_code(&resp_buf);
      LOG_DBG!("rc=0x{:x}", rc);
      rc
    }
    Err(e) => {
      LOG_DBG!("tpm_exchange failed: {}", e);
      0xFFFFFFFF
    }
  }
}
