#![allow(non_snake_case)]

use crate::{
  LOG_DBG,
  tlcl::{tpm_exchange, unmarshal::unmarshal_response_code},
};

#[rustfmt::skip]
const NV_DEFINESPACE_TEMPLATE: [u8; 101] = [
  /* 0-1   */ 0x00, 0xc1,             /* tag: TPM_TAG_RQU_COMMAND */
  /* 2-5   */ 0x00, 0x00, 0x00, 0x65, /* paramSize: 101 bytes */
  /* 6-9   */ 0x00, 0x00, 0x00, 0xcc, /* ordinal: TPM_ORD_NV_DefineSpace */
  
  /* (fun fact: TPM_NV_DATA_PUBLIC starts here) */
  /* 10-11 */ 0x00, 0x18,             /* tag: TPM_TAG_NV_DATA_PUBLIC */
  /* 12-15 */ 0x00, 0x00, 0x00, 0x00, /* nvIndex */

  /* pcrInfoRead: TPM_PCR_INFO_SHORT */
  /* 16-17 */ 0x00, 0x03,             /* pcrSelection.sizeOfSelect: 3 */
  /* 18-20 */ 0x00, 0x00, 0x00,       /* pcrSelection.pcrSelect (3 bytes) */
  /* 21    */ 0x1f,                   /* localityAtRelease (1 byte) */
  /* 22-41 */ 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 
              0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, /* digestAtRelease (20 bytes) */
              
  /* pcrInfoWrite: TPM_PCR_INFO_SHORT */
  /* 42-43 */ 0x00, 0x03,             /* pcrSelection.sizeOfSelect: 3 */
  /* 44-46 */ 0x00, 0x00, 0x00,       /* pcrSelection.pcrSelect (3 bytes) */
  /* 47    */ 0x1f,                   /* localityAtRelease (1 byte) */
  /* 48-67 */ 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 
              0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, /* digestAtRelease (20 bytes) */
  
  /* 68-69 */ 0x00, 0x17,             /* permission.tag: TPM_TAG_NV_ATTRIBUTES */
  /* 70-73 */ 0x00, 0x00, 0x00, 0x00, /* permission.attributes (UINT32) */
  
  /* 74    */ 0x00,                   /* bReadSTClear (TSS_BOOL) */
  /* 75    */ 0x00,                   /* bWriteSTClear (TSS_BOOL) */
  /* 76    */ 0x00,                   /* bWriteDefine (TSS_BOOL) */
  /* 77-80 */ 0x00, 0x00, 0x00, 0x00, /* dataSize (UINT32) */
  
  /* 81-100 */                        /* Padding? Not sure.. */
  0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 
  0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
];

const OFFSET_INDEX: usize = 12; // ends at 15
const OFFSET_PERM: usize = 70; // ends at 73
const OFFSET_SIZE: usize = 77; // ends at 80

fn patch_u32(buf: &mut [u8], offset: usize, val: u32) {
  buf[offset] = (val >> 24) as u8;
  buf[offset + 1] = (val >> 16) as u8;
  buf[offset + 2] = (val >> 8) as u8;
  buf[offset + 3] = (val & 0xFF) as u8;
}

fn define_space_ex(index: u32, perm: u32, size: u32) -> u32 {
  let mut cmd_buf: [u8; 512] = [0; 512];
  let mut resp_buf: [u8; 4096] = [0; 4096];

  cmd_buf[..101].copy_from_slice(&NV_DEFINESPACE_TEMPLATE);
  patch_u32(&mut cmd_buf, OFFSET_INDEX, index);
  patch_u32(&mut cmd_buf, OFFSET_PERM, perm);
  patch_u32(&mut cmd_buf, OFFSET_SIZE, size);

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

pub fn TlclDefineSpace(index: u32, perm: u32, size: u32) -> u32 {
  define_space_ex(index, perm, size)
}

pub fn TlclDefineSpaceEx(
  _owner_auth: Option<&[u8]>, /* auth not implemented for tpm1.2 */
  index: u32,
  perm: u32,
  size: u32,
) -> u32 {
  define_space_ex(index, perm, size)
}

/* TPM 1.2 undefine is just define with perm=0, size=0. */
pub fn TlclUndefineSpace(index: u32) -> u32 {
  define_space_ex(index, 0, 0)
}

pub fn TlclUndefineSpaceEx(_owner_auth: Option<&[u8]>, index: u32) -> u32 {
  define_space_ex(index, 0, 0)
}
