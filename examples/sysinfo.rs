use libcros::sysinfo::{kernver, fwver};
use libcros::libargs::ArgCheck;
use libcros::{LOG, Logger, kv_set, kv_get};

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
  kv_get(libcros::keys::TPM_PATH);

  let kernver: u32 = kernver();
  LOG!("Kernel rollback version: 0x{:08x}", kernver);

  let fwver: u32 = fwver();
  LOG!("Firmware rollback version: 0x{:08x}", fwver);

  /*
  TODO: we need the following:
  - emmc
  - cpu
  - ram
  - gpu(?)
  - battery (incl charge, estimated time remaining, etc)
  - gsc info (version, offsets, etc)
  - ec info (version, chip, etc)
  - wp info
  - ap fw info (version, etc)
  - serial number
  */
}
