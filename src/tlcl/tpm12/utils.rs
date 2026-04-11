#![allow(non_snake_case)]

pub use crate::tlcl::bytes::{read_be16, read_be32, write_be16, write_be32};
use crate::tlcl::tpm12::constants::{
  TPM_ALL_LOCALITIES, TPM_DIGEST, TPM_LOC_THREE, TPM_NV_AUTH_POLICY, TPM_PCR_INFO_SHORT,
  TPM_PCR_SELECTION,
};

const EMPTY_PCR_SELECTION_SHA1: [u8; 20] = [
  0x79, 0xdd, 0xda, 0xfd, 0xc1, 0x97, 0xdc, 0xcc, 0xe9, 0x98, 0x9a, 0xee, 0xf5, 0x52, 0x89, 0xee,
  0x24, 0x96, 0x4c, 0xac,
];

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

pub fn init_default_nv_auth_policy(policy: *mut TPM_NV_AUTH_POLICY) {
  let pcr_info = TPM_PCR_INFO_SHORT {
    pcrSelection: TPM_PCR_SELECTION {
      sizeOfSelect: u16::from_be(3),
      pcrSelect: [0, 0, 0],
    },
    localityAtRelease: (TPM_ALL_LOCALITIES & !TPM_LOC_THREE) as u8,
    digestAtRelease: TPM_DIGEST {
      digest: EMPTY_PCR_SELECTION_SHA1,
    },
  };

  unsafe {
    core::ptr::addr_of_mut!((*policy).pcr_info_read).write_unaligned(pcr_info);
    core::ptr::addr_of_mut!((*policy).pcr_info_write).write_unaligned(pcr_info);
  }
}

pub fn init_define_space_default_auth_policy(policy: *mut TPM_NV_AUTH_POLICY) {
  let pcr_info = TPM_PCR_INFO_SHORT {
    pcrSelection: TPM_PCR_SELECTION {
      sizeOfSelect: u16::from_be(3),
      pcrSelect: [0, 0, 0],
    },
    localityAtRelease: TPM_ALL_LOCALITIES as u8,
    digestAtRelease: TPM_DIGEST { digest: [0u8; 20] },
  };

  unsafe {
    core::ptr::addr_of_mut!((*policy).pcr_info_read).write_unaligned(pcr_info);
    core::ptr::addr_of_mut!((*policy).pcr_info_write).write_unaligned(pcr_info);
  }
}
