use libcros::{LOG, LOG_FATAL, Logger, kv_get, kv_set, libargs::ArgCheck, tlcl::TlclSaveState};

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

  LOG!("save state on {:?}", tpm);
  let rc = TlclSaveState();
  if rc != 0 {
    LOG_FATAL!(rc.try_into().unwrap(); "TlclSaveState failed with error code: {:x}", rc);
  }
}
