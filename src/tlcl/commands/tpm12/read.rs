#![allow(non_snake_case)]

use crate::{
  LOG_DBG,
  tlcl::{tpm_exchange, tpm12::constants::*, unmarshal::unmarshal_response_code},
};

#[rustfmt::skip]
const NV_READ_TEMPLATE: [u8; 22] = [
  /* 0-1   */ 0x00, 0xc1,             /* tag: TPM_TAG_RQU_COMMAND */
  /* 2-5   */ 0x00, 0x00, 0x00, 0x16, /* paramSize: 22 */
  /* 6-9   */ 0x00, 0x00, 0x00, 0xcf, /* ordinal: TPM_ORD_NV_ReadValue */
  /* 10-13 */ 0x00, 0x00, 0x00, 0x00, /* nvIndex */
  /* 14-17 */ 0x00, 0x00, 0x00, 0x00, /* offset */
  /* 18-21 */ 0x00, 0x00, 0x00, 0x00  /* dataSize */
];

const OFFSET_INDEX: usize = 10; // ends at 13
const OFFSET_OFFSET: usize = 14; // ends at 17
const OFFSET_SIZE: usize = 18; // ends at 21

fn patch_u32(buf: &mut [u8], offset: usize, val: u32) {
  buf[offset] = (val >> 24) as u8;
  buf[offset + 1] = (val >> 16) as u8;
  buf[offset + 2] = (val >> 8) as u8;
  buf[offset + 3] = (val & 0xFF) as u8;
}

pub fn TlclRead(index: u32, size: u16) -> Vec<u8> {
  TlclReadWithOffset(index, size, 0)
}

pub fn TlclReadWithOffset(index: u32, size: u16, offset: u16) -> Vec<u8> {
  let mut cmd_buf: [u8; 512] = [0; 512];
  let mut resp_buf: [u8; 4096] = [0; 4096];

  cmd_buf[..22].copy_from_slice(&NV_READ_TEMPLATE);
  patch_u32(&mut cmd_buf, OFFSET_INDEX, index);
  patch_u32(&mut cmd_buf, OFFSET_OFFSET, offset as u32);
  patch_u32(&mut cmd_buf, OFFSET_SIZE, size as u32);

  LOG_DBG!("index=0x{:x}, size={}, offset={}", index, size, offset);

  match tpm_exchange(&mut cmd_buf, &mut resp_buf) {
    Ok(_) => {
      let rc = unmarshal_response_code(&resp_buf);
      if rc != TPM_SUCCESS {
        LOG_DBG!("rc=0x{:x}", rc);
        return Vec::new();
      }
      let data_size =
        u32::from_be_bytes([resp_buf[10], resp_buf[11], resp_buf[12], resp_buf[13]]) as usize;
      let read_len = data_size.min(size as usize);
      resp_buf[14..14 + read_len].to_vec()
    }
    Err(e) => {
      LOG_DBG!("tpm_exchange failed: {}", e);
      Vec::new()
    }
  }
}
