use libcros::{
  LOG, LOG_FATAL, Logger, kv_get, kv_set,
  libargs::ArgCheck,
  tlcl::{
    TlclAssertPhysicalPresence, TlclForceClear, TlclPhysicalPresenceCMDEnable, TlclSetEnable,
  },
};

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
  LOG!("clearing {}", &tpm);

  let rc = TlclPhysicalPresenceCMDEnable();
  if rc != 0 {
    LOG_FATAL!(rc.try_into().unwrap(); "TlclPhysicalPresenceCMDEnable failed with error code: {:x}", rc);
  }

  let rc = TlclAssertPhysicalPresence();
  if rc != 0 {
    LOG_FATAL!(rc.try_into().unwrap(); "TlclAssertPhysicalPresence failed with error code: {:x}", rc);
  }

  /* on tpm2.0, this is the only one needed, everything else
  is for tpm 1.2 */
  let rc = TlclForceClear();
  if rc != 0 {
    LOG_FATAL!(rc.try_into().unwrap(); "TlclForceClear failed with error code: {:x}", rc);
  }

  let rc = TlclSetEnable();
  if rc != 0 {
    LOG_FATAL!(rc.try_into().unwrap(); "TlclSetEnable failed with error code: {:x}", rc);
  }
}
