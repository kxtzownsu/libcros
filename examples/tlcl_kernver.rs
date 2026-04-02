#[cfg(feature = "tpm1_2")]
use libcros::tlcl::permissions::{NV_PERM_PPREAD, NV_PERM_PPWRITE};
use libcros::{
  LOG, LOG_FATAL, Logger, kv_set,
  libargs::ArgCheck,
  tlcl::{TlclDefineSpace, TlclRead, TlclUndefineSpace, TlclWrite},
};

const NV_INDEX: u32 = 0x1008;

/* kernver 1 */
const KERN_VER: &[u8] = &[
  0x02, 0x4c, 0x57, 0x52, 0x47, 0x01, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x55,
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
    kv_set(libcros::keys::TPM_PATH, "/dev/tpm69");
  } else {
    kv_set(libcros::keys::TPM_PATH, &flags_tpm_path);
  }

  Logger::init(verbose, true);

  /* platform hierarchy on TPM 1.2 requires physical presence */
  #[cfg(feature = "tpm1_2")]
  {
    use libcros::tlcl::{TlclAssertPhysicalPresence, TlclPhysicalPresenceCMDEnable};
    let rc = TlclPhysicalPresenceCMDEnable();
    if rc != 0 {
      LOG_FATAL!("PhysicalPresenceCMDEnable failed: 0x{:X}", rc);
    }
    let rc = TlclAssertPhysicalPresence();
    if rc != 0 {
      LOG_FATAL!("AssertPhysicalPresence failed: 0x{:X}", rc);
    }
  }

  /* tpm2_nvdefine --hierarchy=p --size=13 0x1008
  platform hierarchy = PLATFORMCREATE on tpm2.0, PPWRITE|PPREAD on tpm1.2 */
  #[cfg(feature = "tpm2_0")]
  let perm = libcros::tlcl::permissions::NV_PERM_PLATFORMCREATE;
  #[cfg(feature = "tpm1_2")]
  let perm = NV_PERM_PPWRITE | NV_PERM_PPREAD;

  LOG!("undefining NV index 0x{:X}", NV_INDEX);
  let rc = TlclUndefineSpace(NV_INDEX);
  LOG!("UndefineSpace rc: 0x{:X}", rc);

  LOG!(
    "defining NV index 0x{:X} with size {} and perm 0x{:X}",
    NV_INDEX,
    KERN_VER.len(),
    perm
  );
  let rc = TlclDefineSpace(NV_INDEX, perm, KERN_VER.len() as u32);
  if rc != 0 {
    LOG_FATAL!("DefineSpace failed: 0x{:X}", rc);
  }
  LOG!("DefineSpace rc: 0x{:X}", rc);

  /* tpm2_nvwrite --hierarchy=p --input=kernver 0x1008 */
  LOG!("writing {} bytes to 0x{:X}", KERN_VER.len(), NV_INDEX);
  let rc = TlclWrite(NV_INDEX, KERN_VER);
  if rc != 0 {
    LOG_FATAL!("Write failed: 0x{:X}", rc);
  }
  LOG!("Write rc: 0x{:X}", rc);

  let kernver = TlclRead(NV_INDEX, KERN_VER.len() as u16);

  if kernver.is_empty() {
    LOG!("Read failed or returned no data.");
  } else {
    let hex_string: String = kernver.iter().map(|b| format!("{:02x} ", b)).collect();
    LOG!("Kernel Version (hex): {}", hex_string);
  }
}
