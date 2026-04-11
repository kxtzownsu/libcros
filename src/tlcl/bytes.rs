pub fn read_be16(src: *const u8) -> u16 {
  unsafe { ((*src.add(0) as u16) << 8) | (*src.add(1) as u16) }
}

pub fn read_be32(src: *const u8) -> u32 {
  unsafe {
    ((*src.add(0) as u32) << 24)
      | ((*src.add(1) as u32) << 16)
      | ((*src.add(2) as u32) << 8)
      | (*src.add(3) as u32)
  }
}

pub fn write_be16(dest: *mut u8, val: u16) {
  unsafe {
    *dest.add(0) = (val >> 8) as u8;
    *dest.add(1) = val as u8;
  }
}

pub fn write_be32(dest: *mut u8, val: u32) {
  unsafe {
    *dest.add(0) = (val >> 24) as u8;
    *dest.add(1) = (val >> 16) as u8;
    *dest.add(2) = (val >> 8) as u8;
    *dest.add(3) = val as u8;
  }
}
