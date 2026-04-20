pub mod constants;

#[cfg(feature = "tlcl")]
pub use crate::tlcl::tpm20;

#[cfg(not(feature = "tlcl"))]
pub mod tpm20 {
  pub mod types {
      pub const TPM_RC_BAD_TAG: u32 = 0x01E;
  }

  pub mod constants {
    pub const TPM_ST_NO_SESSIONS: u16 = 0x8001;
  }
}

use std::io::{Read, Write};

use crate::{LOG_DBG, LOG_FATAL_NOEXIT};
use constants::{
  vendor_cmd_cc, vendor_cmd_rc, CONFIG_EXTENSION_COMMAND, MAX_RX_BUF_SIZE, MAX_TX_BUF_SIZE,
  TPM_CC_VENDOR_BIT_MASK, TPM_ORDINAL_OFFSET, TPM_PKT_CMD_HEADER_SIZE, TPM_PKT_UPG_HEADER_SIZE,
  TPM_SUBCMD_OFFSET, VENDOR_RC_ERR,
};

pub fn send_command<W: Write>(
  tpm: &mut W,
  digest: u32,
  addr: u32,
  data: &[u8],
  subcmd: vendor_cmd_cc,
) -> bool {
  let mut outbuf = [0u8; MAX_TX_BUF_SIZE];
  let cc = subcmd as u32;

  outbuf[0..2].copy_from_slice(&tpm20::constants::TPM_ST_NO_SESSIONS.to_be_bytes());

  let ordinal = if cc <= vendor_cmd_cc::LAST_EXTENSION_COMMAND as u32 {
    CONFIG_EXTENSION_COMMAND
  } else {
    TPM_CC_VENDOR_BIT_MASK
  };
  outbuf[TPM_ORDINAL_OFFSET..TPM_ORDINAL_OFFSET + 4].copy_from_slice(&ordinal.to_be_bytes());
  outbuf[TPM_SUBCMD_OFFSET..TPM_SUBCMD_OFFSET + 2].copy_from_slice(&(cc as u16).to_be_bytes());

  let header_size = if subcmd == vendor_cmd_cc::EXTENSION_FW_UPGRADE {
    outbuf[TPM_PKT_CMD_HEADER_SIZE..TPM_PKT_CMD_HEADER_SIZE + 4]
      .copy_from_slice(&digest.to_ne_bytes());
    outbuf[TPM_PKT_CMD_HEADER_SIZE + 4..TPM_PKT_CMD_HEADER_SIZE + 8]
      .copy_from_slice(&addr.to_be_bytes());
    TPM_PKT_UPG_HEADER_SIZE
  } else {
    TPM_PKT_CMD_HEADER_SIZE
  };

  let len = header_size + data.len();
  outbuf[2..6].copy_from_slice(&(len as u32).to_be_bytes());
  outbuf[header_size..len].copy_from_slice(data);

  match tpm.write_all(&outbuf[..len]) {
    Ok(_) => true,
    Err(e) => {
      LOG_FATAL_NOEXIT!("Failed to write to GSC: {}", e);
      false
    }
  }
}

pub fn read_response<R: Read>(tpm: &mut R, response: &mut [u8]) -> vendor_cmd_rc {
  let mut raw = [0u8; MAX_RX_BUF_SIZE + TPM_PKT_UPG_HEADER_SIZE];
  let mut len = 0usize;

  loop {
    match tpm.read(&mut raw[len..]) {
      Ok(0) => break,
      Ok(n) => len += n,
      Err(e) => {
        LOG_FATAL_NOEXIT!("Failed to read from GSC: {}", e);
        return VENDOR_RC_ERR;
      }
    }
  }

  if len < TPM_PKT_CMD_HEADER_SIZE {
    LOG_FATAL_NOEXIT!("Problems reading from GSC, got {} bytes.", len);
    return VENDOR_RC_ERR;
  }

  let data_len = len - TPM_PKT_CMD_HEADER_SIZE;
  let copy_len = data_len.min(response.len());
  response[..copy_len]
    .copy_from_slice(&raw[TPM_PKT_CMD_HEADER_SIZE..TPM_PKT_CMD_HEADER_SIZE + copy_len]);

  let rv_bytes = [
    raw[TPM_ORDINAL_OFFSET],
    raw[TPM_ORDINAL_OFFSET + 1],
    raw[TPM_ORDINAL_OFFSET + 2],
    raw[TPM_ORDINAL_OFFSET + 3],
  ];
  let rv = u32::from_be_bytes(rv_bytes);
  let rv = if (rv & VENDOR_RC_ERR) == VENDOR_RC_ERR {
    rv & !VENDOR_RC_ERR
  } else {
    rv
  };

  LOG_DBG!("GSC response code: 0x{:x}", rv);
  rv
}