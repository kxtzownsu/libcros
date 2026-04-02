#![allow(non_snake_case)]

use crate::{
  LOG_DBG,
  structs::{Tpm2SessionHeader, Tpm2TpmHeader},
  tlcl::{
    marshal::{marshal_session_header, marshal_tpm_header, marshal_u16, marshal_u32},
    tpm_exchange,
    tpm20::constants::{
      HR_NV_INDEX, TPM_CMD_HEADER_SIZE, TPM_RH_PLATFORM, TPM_RS_PW, TPM_ST_SESSIONS, TPM2_NV_Write,
      TPMI_RH_NV_INDEX_OWNER_START,
    },
    unmarshal::unmarshal_response_code,
  },
};

fn nv_write_auth(nv_index: u32) -> u32 {
  if nv_index >= TPMI_RH_NV_INDEX_OWNER_START {
    nv_index
  } else {
    TPM_RH_PLATFORM
  }
}

fn serialize_nv_write(nv_index: u32, data: &[u8], buf: &mut [u8; 512]) -> usize {
  let mut offset = TPM_CMD_HEADER_SIZE;
  let mut max_size = 512 - TPM_CMD_HEADER_SIZE;
  let mut remaining = max_size;

  marshal_u32(nv_write_auth(nv_index), &mut offset, buf, &mut remaining);
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

  marshal_u16(data.len() as u16, &mut offset, buf, &mut remaining);
  buf[offset..offset + data.len()].copy_from_slice(data);
  offset += data.len();
  remaining -= data.len();

  marshal_u16(0, &mut offset, buf, &mut remaining);

  let total_size = (max_size - remaining + TPM_CMD_HEADER_SIZE) as u32;
  marshal_tpm_header(
    Tpm2TpmHeader {
      tpm_tag: TPM_ST_SESSIONS,
      tpm_size: total_size,
      tpm_code: TPM2_NV_Write,
    },
    buf,
    &mut max_size,
    &mut 0,
  );

  total_size as usize
}

pub fn TlclWrite(index: u32, data: &[u8]) -> u32 {
  let nv_index = HR_NV_INDEX + index;

  if data.len() + 64 > 512 {
    LOG_DBG!("data too large ({} bytes)", data.len());
    return 0xFFFFFFFF;
  }

  let mut cmd_buf: [u8; 512] = [0; 512];
  let mut resp_buf: [u8; 4096] = [0; 4096];

  serialize_nv_write(nv_index, data, &mut cmd_buf);
  LOG_DBG!("index=0x{:x}, len={}", index, data.len());

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
