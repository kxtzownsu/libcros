#[cfg(feature = "tpm2_0")]
use libcros::tlcl::tpm20::types::TPM_RC_INITIALIZE;
use libcros::{kv_get, kv_set, libargs::ArgCheck, tlcl::TlclResume, Logger, LOG, LOG_FATAL};

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

  LOG!("resume on {}", tpm);
  let rc = TlclResume();
  #[cfg(feature = "tpm2_0")]
  if rc == TPM_RC_INITIALIZE {
    LOG!("resume skipped: already initialized (rc={:x})", rc);
    return;
  }
  if rc != 0 {
    LOG_FATAL!(rc.try_into().unwrap(); "TlclResume failed with error code: {:x}", rc);
  }
}
