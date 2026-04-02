/* response header layout is the same on TPM1.2 and TPM2.0:
bytes 0-1: tag, bytes 2-5: size, bytes 6-9: response code */

pub fn unmarshal_u32(buf: &[u8; 4096], offset: &mut usize) -> u32 {
  let v = ((buf[*offset] as u32) << 24)
    | ((buf[*offset + 1] as u32) << 16)
    | ((buf[*offset + 2] as u32) << 8)
    | (buf[*offset + 3] as u32);
  *offset += 4;
  v
}

pub fn unmarshal_u16(buf: &[u8; 4096], offset: &mut usize) -> u16 {
  let v = ((buf[*offset] as u16) << 8) | (buf[*offset + 1] as u16);
  *offset += 2;
  v
}

pub fn unmarshal_u8(buf: &[u8; 4096], offset: &mut usize) -> u8 {
  let v = buf[*offset];
  *offset += 1;
  v
}

/* response code is always at bytes 6-9 (nice) */
pub fn unmarshal_response_code(buf: &[u8; 4096]) -> u32 {
  let mut offset = 0;
  let _tag = unmarshal_u16(buf, &mut offset);
  let _size = unmarshal_u32(buf, &mut offset);
  unmarshal_u32(buf, &mut offset)
}
