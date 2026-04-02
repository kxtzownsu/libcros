#![allow(non_snake_case)]

use std::{mem, mem::ManuallyDrop};

use crate::{
  LOG_DBG,
  structs::{Tpm2SessionHeader, Tpm2TpmHeader},
  tlcl::{
    marshal::{marshal_session_header, marshal_tpm_header, marshal_u16, marshal_u32},
    structs::{NvReadResponse, Tpm2Response},
    tpm_exchange,
    tpm20::constants::{
      HR_NV_INDEX, TPM_CMD_HEADER_SIZE, TPM_RC_SUCCESS, TPM_RH_PLATFORM, TPM_RS_PW,
      TPM_ST_NO_SESSIONS, TPM_ST_SESSIONS, TPM2_NV_Read, TPM2_NV_ReadPublic,
      TPMA_NV_PLATFORMCREATE,
    },
    unmarshal::{unmarshal_response_code, unmarshal_u16, unmarshal_u32},
  },
};

#[derive(Debug)]
struct NvReadCmd {
  authHandle: u32,
  nvIndex: u32,
  size: u16,
  offset: u16,
}

fn serialize_nv_read(cmd: NvReadCmd, buf: &mut [u8; 512]) {
  let mut offset = TPM_CMD_HEADER_SIZE;
  let mut max_size = 512 - TPM_CMD_HEADER_SIZE;
  let mut remaining = max_size;

  marshal_u32(cmd.authHandle, &mut offset, buf, &mut remaining);
  marshal_u32(cmd.nvIndex, &mut offset, buf, &mut remaining);

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

  marshal_u16(cmd.size, &mut offset, buf, &mut remaining);
  marshal_u16(cmd.offset, &mut offset, buf, &mut remaining);

  let total_size = (max_size - remaining + TPM_CMD_HEADER_SIZE) as u32;
  marshal_tpm_header(
    Tpm2TpmHeader {
      tpm_tag: TPM_ST_SESSIONS,
      tpm_size: total_size,
      tpm_code: TPM2_NV_Read,
    },
    buf,
    &mut max_size,
    &mut 0,
  );
}

fn parse_nv_read_response(buf: &mut [u8; 4096], size: u16, response: &mut Tpm2Response) -> Vec<u8> {
  unsafe {
    let mut offset = 0;
    response.hdr.tpm_tag = unmarshal_u16(buf, &mut offset);
    response.hdr.tpm_size = unmarshal_u32(buf, &mut offset);
    response.hdr.tpm_code = unmarshal_u32(buf, &mut offset);

    let nvr = &mut *(&mut response.data.nvr as *mut ManuallyDrop<NvReadResponse>);
    nvr.params_size = unmarshal_u32(buf, &mut offset);
    nvr.buffer.size = unmarshal_u16(buf, &mut offset);

    let end = offset + size as usize;
    buf[offset..end].to_vec()
  }
}

pub fn TlclRead(index: u32, size: u16) -> Vec<u8> {
  TlclReadWithOffset(index, size, 0)
}

pub fn TlclReadWithOffset(index: u32, size: u16, offset: u16) -> Vec<u8> {
  let permissions = TlclGetPermissions(index);
  let use_platform_auth = permissions & TPMA_NV_PLATFORMCREATE != 0;
  let nv_index = HR_NV_INDEX + index;

  let auth_handle = if use_platform_auth {
    TPM_RH_PLATFORM
  } else {
    nv_index
  };

  let mut cmd_buf: [u8; 512] = [0; 512];
  let mut resp_buf: [u8; 4096] = [0; 4096];

  let cmd = NvReadCmd {
    authHandle: auth_handle,
    nvIndex: nv_index,
    size,
    offset,
  };

  serialize_nv_read(cmd, &mut cmd_buf);

  match tpm_exchange(&mut cmd_buf, &mut resp_buf) {
    Ok(_) => {
      let rc = unmarshal_response_code(&resp_buf);
      if rc != TPM_RC_SUCCESS {
        LOG_DBG!("rc=0x{:x}", rc);
        return Vec::new();
      }
      let mut response: Tpm2Response = unsafe { mem::zeroed() };
      let bytes = parse_nv_read_response(&mut resp_buf, size, &mut response);
      LOG_DBG!("read {} bytes", bytes.len());
      bytes
    }
    Err(e) => {
      LOG_DBG!("tpm_exchange failed: {}", e);
      Vec::new()
    }
  }
}

pub fn TlclGetPermissions(index: u32) -> u32 {
  let nv_index = HR_NV_INDEX + index;
  let mut cmd_buf: [u8; 512] = [0; 512];
  let mut resp_buf: [u8; 4096] = [0; 4096];

  let mut max_size = 512 - TPM_CMD_HEADER_SIZE;
  let mut offset = TPM_CMD_HEADER_SIZE;
  let mut remaining = max_size;
  marshal_u32(nv_index, &mut offset, &mut cmd_buf, &mut remaining);
  let total_size = (max_size - remaining + TPM_CMD_HEADER_SIZE) as u32;
  marshal_tpm_header(
    Tpm2TpmHeader {
      tpm_tag: TPM_ST_NO_SESSIONS,
      tpm_size: total_size,
      tpm_code: TPM2_NV_ReadPublic,
    },
    &mut cmd_buf,
    &mut max_size,
    &mut 0,
  );

  match tpm_exchange(&mut cmd_buf, &mut resp_buf) {
    Ok(_) => {
      /* response: header(10) + nvPublic size(2) + nvIndex(4) + nameAlg(2) + attributes(4) */
      let rc = unmarshal_response_code(&resp_buf);
      if rc != TPM_RC_SUCCESS {
        LOG_DBG!("rc=0x{:x}", rc);
        return 0;
      }
      let off = 10 + 2 + 4 + 2; /* skip header + nvPublic.size + nvIndex + nameAlg */
      u32::from_be_bytes([
        resp_buf[off],
        resp_buf[off + 1],
        resp_buf[off + 2],
        resp_buf[off + 3],
      ])
    }
    Err(e) => {
      LOG_DBG!("tpm_exchange failed: {}", e);
      0
    }
  }
}
