#![allow(unused_variables)]

#[cfg(feature = "tpm1_2")]
use libcros::tlcl::tpm12::constants::{TPM_NV_PER_PPREAD, TPM_NV_PER_PPWRITE};
use libcros::{
  LOG, LOG_FATAL, Logger, kv_set,
  libargs::ArgCheck,
  tlcl::{TlclDefineSpace, TlclRead, TlclUndefineSpace, TlclWrite},
};

const NV_INDEX: u32 = 0x1008;

/* kernver 0x00000000 */
const KERN_VER: &[u8] = &[
  0x02, 0x4c, 0x57, 0x52, 0x47, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xe8,
];

fn main() {
  let mut args: ArgCheck = ArgCheck::new();
  let verbose: bool = args.fbool("--verbose", "", "Enable debug messages");
  let flags_tpm_path = args.fequals_str(
    "--tpm-path",
    "-t",
    "Specify a custom TPM device to use in /dev/tpmX format",
  );

  args.check_help();

  if flags_tpm_path.is_empty() {
    kv_set(libcros::keys::TPM_PATH, "/dev/tpm0");
  } else {
    kv_set(libcros::keys::TPM_PATH, &*flags_tpm_path);
  }

  Logger::init(verbose, true);

  /* platform hierarchy on TPM 1.2 requires physical presence */
  #[cfg(feature = "tpm1_2")]
  {
    use libcros::tlcl::{TlclAssertPhysicalPresence, TlclPhysicalPresenceCMDEnable, TlclSetEnable};
    let rc = TlclPhysicalPresenceCMDEnable();
    if rc != 0 {
      LOG_FATAL!("PhysicalPresenceCMDEnable failed: 0x{:X}", rc);
    }
    let rc = TlclAssertPhysicalPresence();
    if rc != 0 {
      LOG_FATAL!("AssertPhysicalPresence failed: 0x{:X}", rc);
    }
    let rc = TlclSetEnable();
    if rc != 0 {
      LOG_FATAL!("TlclSetEnable failed: 0x{:X}", rc);
    }
  }

  /* tpm2_nvdefine --hierarchy=p --size=13 0x1008
  PPWRITE|PPREAD on tpm 1.2, those two plus PLATFORMCREATE on tpm 2.0 */
  #[cfg(feature = "tpm2_0")]
  let perm = libcros::tlcl::tpm20::constants::TPMA_NV_PLATFORMCREATE
    | libcros::tlcl::tpm20::constants::TPMA_NV_PPWRITE
    | libcros::tlcl::tpm20::constants::TPMA_NV_PPREAD;
  #[cfg(feature = "tpm1_2")]
  let perm = TPM_NV_PER_PPWRITE | TPM_NV_PER_PPREAD;

  LOG!("undefining NV index 0x{:X}", NV_INDEX);
  TlclUndefineSpace(NV_INDEX);

  LOG!(
    "defining NV index 0x{:X} with size {} and perm 0x{:X}",
    NV_INDEX,
    KERN_VER.len(),
    perm
  );
  let rc = TlclDefineSpace(NV_INDEX, perm, KERN_VER.len() as u32);
  if rc != 0 {
    LOG_FATAL!(rc.try_into().unwrap(); "TlclDefineSpace failed with error code: {:x}", rc);
  }

  /* tpm2_nvwrite --hierarchy=p --input=kernver 0x1008 */
  LOG!("writing {} bytes to 0x{:X}", KERN_VER.len(), NV_INDEX);
  let rc = TlclWrite(
    NV_INDEX,
    KERN_VER.as_ptr() as *const core::ffi::c_void,
    KERN_VER.len() as u32,
  );
  if rc != 0 {
    LOG_FATAL!(rc.try_into().unwrap(); "TlclWrite failed with error code: {:x}", rc);
  }

  let mut outbuf = [0u8; KERN_VER.len()];
  let rc = TlclRead(
    NV_INDEX,
    outbuf.as_mut_ptr() as *mut core::ffi::c_void,
    KERN_VER.len() as u32,
  );
  if rc != 0 {
    LOG_FATAL!(rc.try_into().unwrap(); "TlclRead failed with error code: {:x}", rc);
  }

  let hex_string: String = outbuf.iter().map(|b| format!("{:02x} ", b)).collect();
  LOG!("kernver (hex): {}", hex_string);
}
