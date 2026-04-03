#![allow(non_snake_case)]

use crate::{
  tlcl::{tpm12::constants::*, tpm_exchange, unmarshal::unmarshal_response_code},
  LOG_DBG,
};

#[rustfmt::skip]
const NV_WRITE_HEADER: [u8; 22] = [
  /* 0-1   */ 0x00, 0xc1,             /* tag: TPM_TAG_RQU_COMMAND */
  /* 2-5   */ 0x00, 0x00, 0x00, 0x00, /* paramSize (must be set to 22 + data len) */
  /* 6-9   */ 0x00, 0x00, 0x00, 0xcd, /* ordinal: TPM_ORD_NV_WriteValue */
  /* 10-13 */ 0x00, 0x00, 0x00, 0x00, /* nvIndex */
  /* 14-17 */ 0x00, 0x00, 0x00, 0x00, /* offset */
  /* 18-21 */ 0x00, 0x00, 0x00, 0x00, /* dataSize */
];

const OFFSET_PARAMSIZE: usize = 2; // ends at 5
const OFFSET_INDEX: usize = 10; // ends at 13
const OFFSET_DATASIZE: usize = 18; // ends at 21

fn patch_u32(buf: &mut [u8], offset: usize, val: u32) {
  buf[offset] = (val >> 24) as u8;
  buf[offset + 1] = (val >> 16) as u8;
  buf[offset + 2] = (val >> 8) as u8;
  buf[offset + 3] = (val & 0xFF) as u8;
}

pub fn TlclWrite(index: u32, data: &[u8]) -> u32 {
  let data_len = data.len();
  let total_size = TPM_CMD_HEADER_SIZE + TPM_WRITE_INFO_SIZE + data_len;

  if total_size > 512 {
    LOG_DBG!("data too large ({} bytes)", data_len);
    return 0xFFFFFFFF;
  }

  let mut cmd_buf: [u8; 512] = [0; 512];
  let mut resp_buf: [u8; 4096] = [0; 4096];

  cmd_buf[..22].copy_from_slice(&NV_WRITE_HEADER);
  patch_u32(&mut cmd_buf, OFFSET_PARAMSIZE, total_size as u32);
  patch_u32(&mut cmd_buf, OFFSET_INDEX, index);
  patch_u32(&mut cmd_buf, OFFSET_DATASIZE, data_len as u32);
  cmd_buf[22..22 + data_len].copy_from_slice(data);

  LOG_DBG!("index=0x{:x}, len={}", index, data_len);

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
