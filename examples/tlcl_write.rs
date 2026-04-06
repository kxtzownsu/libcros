use libcros::{LOG, LOG_FATAL, Logger, kv_get, kv_set, libargs::ArgCheck, tlcl::TlclWrite};

const NV_INDEX: u32 = 0x1008;
// kernver 0x00010002
const DATA: [u8; 13] = [
  0x02, 0x4C, 0x57, 0x52, 0x47, 0x02, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x33,
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
    kv_set(libcros::keys::TPM_PATH, &flags_tpm_path);
  }

  Logger::init(verbose, true);
  let tpm = kv_get(libcros::keys::TPM_PATH);

  LOG!(
    "writing {} bytes to index {} on {}",
    DATA.len(),
    NV_INDEX,
    tpm
  );
  let rc = TlclWrite(
    NV_INDEX,
    DATA.as_ptr() as *const core::ffi::c_void,
    DATA.len() as u32,
  );
  if rc != 0 {
    LOG_FATAL!(rc.try_into().unwrap(); "TlclWrite failed with error code: {:x}", rc);
  }
  LOG!("write succeeded");
}
