#![allow(non_snake_case)]

use crate::{
  tlcl::{tpm_exchange, unmarshal::unmarshal_response_code},
  LOG_DBG,
};

#[rustfmt::skip]
const PP_CMD_ENABLE: [u8; 12] = [
  /* 0-1   */ 0x00, 0xc1,             /* tag: TPM_TAG_RQU_COMMAND */
  /* 2-5   */ 0x00, 0x00, 0x00, 0x0c, /* paramSize: 12 */
  /* 6-9   */ 0x40, 0x00, 0x00, 0x0a, /* ordinal: TSC_ORD_PhysicalPresence */
  /* 10-11 */ 0x00, 0x20,             /* physicalPresence: TPM_PHYSICAL_PRESENCE_CMD_ENABLE */
];

#[rustfmt::skip]
const PP_ASSERT: [u8; 12] = [
  /* 0-1   */ 0x00, 0xc1,             /* tag: TPM_TAG_RQU_COMMAND */
  /* 2-5   */ 0x00, 0x00, 0x00, 0x0c, /* paramSize: 12 */
  /* 6-9   */ 0x40, 0x00, 0x00, 0x0a, /* ordinal: TSC_ORD_PhysicalPresence */
  /* 10-11 */ 0x00, 0x08,             /* physicalPresence: TPM_PHYSICAL_PRESENCE_PRESENT */
];

#[rustfmt::skip]
const PHYSICAL_ENABLE: [u8; 10] = [
  /* 0-1   */ 0x00, 0xc1,             /* tag: TPM_TAG_RQU_COMMAND */
  /* 2-5   */ 0x00, 0x00, 0x00, 0x0a, /* paramSize: 10 */
  /* 6-9   */ 0x00, 0x00, 0x00, 0x6f, /* ordinal: TPM_ORD_PhysicalEnable */
];

pub fn TlclPhysicalPresenceCMDEnable() -> u32 {
  let mut cmd_buf: [u8; 512] = [0; 512];
  let mut resp_buf: [u8; 4096] = [0; 4096];
  cmd_buf[..12].copy_from_slice(&PP_CMD_ENABLE);
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

pub fn TlclAssertPhysicalPresence() -> u32 {
  let mut cmd_buf: [u8; 512] = [0; 512];
  let mut resp_buf: [u8; 4096] = [0; 4096];
  cmd_buf[..12].copy_from_slice(&PP_ASSERT);
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

pub fn TlclSetEnable() -> u32 {
  let mut cmd_buf: [u8; 512] = [0; 512];
  let mut resp_buf: [u8; 4096] = [0; 4096];
  cmd_buf[..10].copy_from_slice(&PHYSICAL_ENABLE);
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
