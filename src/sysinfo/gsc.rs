use crate::{
  gsc::{
    constants::{
      board_id, first_response_pdu, signed_header_version,
      vendor_cmd_cc::{EXTENSION_FW_UPGRADE, VENDOR_CC_GET_BOARD_ID},
    },
    read_response, send_command,
  },
  keyval::{KvValue, key_types, keys, kv_get},
};

const PDU_MAX: first_response_pdu = first_response_pdu {
  return_value: u32::MAX,
  protocol_version: u32::MAX,
  backup_ro_offset: u32::MAX,
  backup_rw_offset: u32::MAX,
  shv: [
    signed_header_version {
      minor: u32::MAX,
      major: u32::MAX,
      epoch: u32::MAX,
    },
    signed_header_version {
      minor: u32::MAX,
      major: u32::MAX,
      epoch: u32::MAX,
    },
  ],
  keyid: [u32::MAX, u32::MAX],
};

const BID_MAX: board_id = board_id {
  board_type: u32::MAX,
  type_inv: u32::MAX,
  flags: u32::MAX,
};

pub fn get_gsc_version() -> first_response_pdu {
  let mut f = match kv_get(key_types::FILE, keys::GSC_SOCKET) {
    Some(KvValue::File(f)) => f,
    _ => return PDU_MAX,
  };

  if !send_command(&mut f, 0, 0, &[], EXTENSION_FW_UPGRADE) {
    return PDU_MAX;
  }

  let mut buf = [0u8; core::mem::size_of::<first_response_pdu>()];
  if read_response(&mut f, &mut buf) != 0 {
    return PDU_MAX;
  }

  unsafe { core::ptr::read_unaligned(buf.as_ptr() as *const first_response_pdu) }
}

pub fn get_gsc_board_id() -> board_id {
  let mut f = match kv_get(key_types::FILE, keys::GSC_SOCKET) {
    Some(KvValue::File(f)) => f,
    _ => return BID_MAX,
  };

  if !send_command(&mut f, 0, 0, &[], VENDOR_CC_GET_BOARD_ID) {
    return BID_MAX;
  }

  let mut buf = [0u8; core::mem::size_of::<board_id>()];
  if read_response(&mut f, &mut buf) != 0 {
    return BID_MAX;
  }

  unsafe { core::ptr::read_unaligned(buf.as_ptr() as *const board_id) }
}
