#![allow(non_snake_case)]

use crate::{
  LOG_DBG,
  structs::{Tpm2SessionHeader, Tpm2TpmHeader},
  tlcl::{
    marshal::{marshal_session_header, marshal_tpm_header, marshal_u32},
    tpm_exchange,
    tpm20::constants::{
      TPM_CMD_HEADER_SIZE, TPM_RH_PLATFORM, TPM_RS_PW, TPM_ST_SESSIONS, TPM2_Clear,
    },
    unmarshal::unmarshal_response_code,
  },
};

fn serialize_clear(buf: &mut [u8; 512]) {
  let mut offset = TPM_CMD_HEADER_SIZE;
  let mut max_size = 512 - TPM_CMD_HEADER_SIZE;
  let mut remaining = max_size;

  marshal_u32(TPM_RH_PLATFORM, &mut offset, buf, &mut remaining);

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
      tpm_code: TPM2_Clear,
    },
    buf,
    &mut max_size,
    &mut 0,
  );
}

pub fn TlclForceClear() -> u32 {
  let mut cmd_buf: [u8; 512] = [0; 512];
  let mut resp_buf: [u8; 4096] = [0; 4096];

  serialize_clear(&mut cmd_buf);

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
