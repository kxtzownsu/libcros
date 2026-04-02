/* We only need to marshal with TPM 2.0 (at the moment) */
#[cfg(feature = "tpm2_0")]
use crate::structs::{Tpm2SessionHeader, Tpm2TpmHeader};

pub fn marshal_u32(val: u32, offset: &mut usize, buf: &mut [u8; 512], size: &mut usize) {
  buf[*offset] = (val >> 24) as u8;
  buf[*offset + 1] = (val >> 16) as u8;
  buf[*offset + 2] = (val >> 8) as u8;
  buf[*offset + 3] = (val & 0xFF) as u8;
  *offset += 4;
  *size -= std::mem::size_of_val(&val);
}

pub fn marshal_u16(val: u16, offset: &mut usize, buf: &mut [u8; 512], size: &mut usize) {
  buf[*offset] = (val >> 8) as u8;
  buf[*offset + 1] = (val & 0xFF) as u8;
  *offset += 2;
  *size -= std::mem::size_of_val(&val);
}

pub fn marshal_u8(val: u8, offset: &mut usize, buf: &mut [u8; 512], size: &mut usize) {
  buf[*offset] = val;
  *offset += 1;
  *size -= std::mem::size_of_val(&val);
}

pub fn marshal_session_header(
  hdr: Tpm2SessionHeader,
  buf: &mut [u8; 512],
  size: &mut usize,
  offset: &mut usize,
) {
  let mut len_field_offset = *offset;
  *offset += 4;

  let body_start = *offset;
  marshal_u32(hdr.session_handle, offset, buf, size);
  marshal_u16(hdr.nonce_size, offset, buf, size);
  marshal_u8(hdr.session_attrs, offset, buf, size);
  marshal_u16(hdr.auth_size, offset, buf, size);
  let body_end = *offset;

  marshal_u32(
    (body_end - body_start) as u32,
    &mut len_field_offset,
    buf,
    size,
  );
}

pub fn marshal_tpm_header(
  hdr: Tpm2TpmHeader,
  buf: &mut [u8; 512],
  size: &mut usize,
  offset: &mut usize,
) {
  marshal_u16(hdr.tpm_tag, offset, buf, size);
  marshal_u32(hdr.tpm_size, offset, buf, size);
  marshal_u32(hdr.tpm_code, offset, buf, size);
}
