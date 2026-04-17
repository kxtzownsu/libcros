use libcros::sysinfo::{get_kernel_rollback_version, get_firmware_rollback_version, get_tpm_version, get_firmware_management_parameters};
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
  let ph_disabled: bool = args.fbool("--disable-platform-hierarchy", "-p", "disable platform hierarchy");

  args.check_help();

  if flags_tpm_path.is_empty() {
    kv_set(libcros::keys::TPM_PATH, "/dev/tpm0");
  } else {
    kv_set(libcros::keys::TPM_PATH, &flags_tpm_path);
  }

  if ph_disabled {
    kv_set(libcros::keys::PH_DISABLED, true);
  } else {
    kv_set(libcros::keys::PH_DISABLED, false);
  }

  Logger::init(verbose, true);
  kv_get(libcros::keys::TPM_PATH);

  let tpm_version: String = get_tpm_version();
  LOG!("TPM version: {}", tpm_version);

  let kernver: u32 = get_kernel_rollback_version();
  LOG!("Kernel rollback version: 0x{:08x}", kernver);

  let fwver: u32 = get_firmware_rollback_version();
  LOG!("Firmware rollback version: 0x{:08x}", fwver);

  let fwmp: u32 = get_firmware_management_parameters();
  if fwmp == u32::MAX {
    LOG!("FWMP index doesn't exist!")
  } else {
    LOG!("FWMP: 0x{:08x}", fwmp);
  }
  
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
