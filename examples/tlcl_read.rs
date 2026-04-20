use libcros::{LOG, LOG_FATAL, Logger, kv_get, kv_set, libargs::ArgCheck, tlcl::TlclRead};

const NV_INDEX: u32 = 0x1008;
const SIZE: usize = 0xD;

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
  let tpm = kv_get(libcros::key_types::STRING, libcros::keys::TPM_PATH);

  LOG!(
    "reading {} bytes from index {} on {:?}",
    SIZE,
    NV_INDEX,
    tpm
  );
  let mut outbuf = [0u8; SIZE];
  let rc = TlclRead(
    NV_INDEX,
    outbuf.as_mut_ptr() as *mut core::ffi::c_void,
    SIZE.try_into().unwrap(),
  );
  if rc != 0 {
    LOG_FATAL!(rc.try_into().unwrap(); "TlclRead failed with error code: {:x}", rc);
  }
  LOG!("read output: {:02X?}", outbuf);
}
