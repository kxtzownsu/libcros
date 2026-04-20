use libcros::sysinfo::backend::{open_gsc_socket, close_gsc_socket};
use libcros::sysinfo::{get_kernel_rollback_version, get_firmware_rollback_version, get_tpm_version, get_firmware_management_parameters, get_gsc_version, get_gsc_board_id};
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
  #[cfg(feature = "tpm2_0")]
  let ph_disabled: bool = args.fbool("--disable-platform-hierarchy", "-p", "disable platform hierarchy");

  args.check_help();

  if flags_tpm_path.is_empty() {
    kv_set(libcros::keys::TPM_PATH, "/dev/tpm0");
  } else {
    kv_set(libcros::keys::TPM_PATH, &*flags_tpm_path);
  }

  #[cfg(feature = "tpm2_0")]
  if ph_disabled {
    kv_set(libcros::keys::PH_DISABLED, true);
  } else {
    kv_set(libcros::keys::PH_DISABLED, false);
  }

  Logger::init(verbose, true);
  kv_get(libcros::key_types::STRING, libcros::keys::TPM_PATH);

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

  /* Before doing anything, we must open the GSC socket.. */
  open_gsc_socket();

  let fpdu = get_gsc_version();
  LOG!("fpdu: {:x?}", fpdu);

  let bid = get_gsc_board_id();
  LOG!("Board ID: {:x?}", bid);

  /* Now that we're done, we can close our connection to the GSC. */
  close_gsc_socket();
  
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
