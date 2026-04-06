#![allow(non_snake_case)]

use crate::tlcl::tpm12::constants::{TPM_DIGEST, TPM_PCR_INFO_SHORT, TPM_PCR_SELECTION};

pub fn read_be16(src: *const u8) -> u16 {
  unsafe { ((*src.add(0) as u16) << 8) | ((*src.add(1) as u16) << 0) }
}

pub fn read_be32(src: *const u8) -> u32 {
  unsafe {
    ((*src.add(0) as u32) << 24)
      | ((*src.add(1) as u32) << 16)
      | ((*src.add(2) as u32) << 8)
      | ((*src.add(3) as u32) << 0)
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

pub fn decode_pcr_info(
  response: &[u8],
  cursor: &mut usize,
  end: usize,
  pcr_info: *mut TPM_PCR_INFO_SHORT,
) -> bool {
  if end.saturating_sub(*cursor) < core::mem::size_of::<u16>() {
    return false;
  }

  let size_of_select = read_be16(unsafe { response.as_ptr().add(*cursor) });
  let encoded_size = core::mem::size_of::<TPM_PCR_INFO_SHORT>() - 3 + size_of_select as usize;
  if end.saturating_sub(*cursor) < encoded_size || size_of_select as usize > 3 {
    return false;
  }

  let mut pcr_select = [0u8; 3];
  let select_start = *cursor + core::mem::size_of::<u16>();
  pcr_select[..size_of_select as usize]
    .copy_from_slice(&response[select_start..select_start + size_of_select as usize]);
  *cursor += core::mem::size_of::<u16>() + size_of_select as usize;

  if end.saturating_sub(*cursor) < 1 + core::mem::size_of::<TPM_DIGEST>() {
    return false;
  }

  let locality = response[*cursor];
  *cursor += 1;

  let mut digest = [0u8; core::mem::size_of::<TPM_DIGEST>()];
  digest.copy_from_slice(&response[*cursor..*cursor + core::mem::size_of::<TPM_DIGEST>()]);
  *cursor += core::mem::size_of::<TPM_DIGEST>();

  let info = TPM_PCR_INFO_SHORT {
    pcrSelection: TPM_PCR_SELECTION {
      sizeOfSelect: size_of_select,
      pcrSelect: pcr_select,
    },
    localityAtRelease: locality,
    digestAtRelease: TPM_DIGEST { digest },
  };

  unsafe {
    pcr_info.write_unaligned(info);
  }
  true
}
