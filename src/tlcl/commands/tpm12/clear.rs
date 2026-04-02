#![allow(non_snake_case)]

use crate::{
  LOG_DBG,
  tlcl::{tpm_exchange, unmarshal::unmarshal_response_code},
};

#[rustfmt::skip]
const FORCE_CLEAR_CMD: [u8; 10] = [
  /* 0-1 */ 0x00, 0xc1,             /* tag: TPM_TAG_RQU_COMMAND */
  /* 2-5 */ 0x00, 0x00, 0x00, 0x0a, /* paramSize: 10 */
  /* 6-9 */ 0x00, 0x00, 0x00, 0x5d, /* ordinal: TPM_ORD_ForceClear */
];

pub fn TlclForceClear() -> u32 {
  let mut cmd_buf: [u8; 512] = [0; 512];
  let mut resp_buf: [u8; 4096] = [0; 4096];

  cmd_buf[..10].copy_from_slice(&FORCE_CLEAR_CMD);

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
