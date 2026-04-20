#[cfg(feature = "tlcl")]
pub use libcros::tlcl::tpm20;
use libcros::{
  LOG, Logger, kv_get, kv_set,
  libargs::ArgCheck,
  sysinfo::{
    backend::{close_gsc_socket, open_gsc_socket},
    get_firmware_management_parameters, get_firmware_rollback_version, get_gsc_board_id,
    get_gsc_version, get_kernel_rollback_version, get_tpm_version,
  },
};

#[cfg(not(feature = "tlcl"))]
pub mod tpm20 {
  pub mod types {
    pub const TPM_RC_1: u32 = 0x100;
    pub const TPM_RC_HANDLE: u32 = 0x08B;
  }
}

fn main() {
  let mut args: ArgCheck = ArgCheck::new();
  let verbose: bool = args.fbool("--verbose", "", "Enable debug messages");
  let flags_tpm_path = args.fequals_str(
    "--tpm-path",
    "-t",
    "Specify a custom TPM device to use in /dev/tpmX format",
  );
  #[cfg(feature = "tpm2_0")]
  let ph_disabled: bool = args.fbool(
    "--disable-platform-hierarchy",
    "-p",
    "disable platform hierarchy",
  );

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
  let kernver = get_kernel_rollback_version();
  let fwver = get_firmware_rollback_version();
  let fwmp = get_firmware_management_parameters();

  /* Before doing anything GSC-related, but after doing our regular TPM activities, we must open the GSC socket.. */
  open_gsc_socket();
  let version: libcros::gsc::constants::first_response_pdu = get_gsc_version();
  let bid: libcros::gsc::constants::board_id = get_gsc_board_id();

  let keyid_ro = u32::from_be(version.keyid[0]);
  let keyid_rw = u32::from_be(version.keyid[1]);
  let backup_ro = u32::from_be(version.backup_ro_offset);
  let backup_rw = u32::from_be(version.backup_rw_offset);
  let shv_ro = (
    version.shv[0].epoch,
    u32::from_be(version.shv[0].major),
    u32::from_be(version.shv[0].minor),
  );
  let shv_rw = (
    version.shv[1].epoch,
    u32::from_be(version.shv[1].major),
    u32::from_be(version.shv[1].minor),
  );

  let board_type: u32 = u32::from_be(bid.board_type);
  let type_inv: u32 = u32::from_be(bid.type_inv);
  let flags: u32 = u32::from_be(bid.flags);

  /* Now that we're done, we can close our connection to the GSC. */
  close_gsc_socket();

  LOG!("TPM version: {}", tpm_version);
  LOG!(
    "Kernel rollback version: 0x{:08x}",
    kernver.rollback_version
  );
  LOG!(
    "Firmware rollback version: 0x{:08x}",
    fwver.rollback_version
  );

  if fwmp.rc == tpm20::types::TPM_RC_HANDLE | tpm20::types::TPM_RC_1 {
    LOG!("FWMP index doesn't exist!")
  } else {
    /* This is actually FWMP flags, but it's stored in rollback_version */
    LOG!("FWMP: 0x{:08x}", fwmp.rollback_version);
  }

  LOG!("Key IDs:");
  LOG!(" - RO: {:x}", keyid_ro);
  LOG!(" - RW: {:x}", keyid_rw);
  LOG!("Backup Offsets:");
  LOG!(" - Inactive RO: {:x}", backup_ro);
  LOG!(" - Inactive RW: {:x}", backup_rw);
  LOG!("Versions:");
  LOG!(" - RO: {}.{}.{}", shv_ro.0, shv_ro.1, shv_ro.2);
  LOG!(" - RW: {}.{}.{}", shv_rw.0, shv_rw.1, shv_rw.2);

  LOG!(
    "Board ID: {:08x}:{:08x}:{:08x}",
    board_type,
    type_inv,
    flags
  );

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
