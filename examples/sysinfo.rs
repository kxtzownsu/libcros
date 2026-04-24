use std::fs::File;

use libcros::{
  LOG, Logger, kv_get, kv_set,
  diskutils::gpt::read_header,
  libargs::ArgCheck,
  sysinfo::{
    backend::{close_gsc_socket, open_gsc_socket},
    get_firmware_management_parameters, get_firmware_rollback_version, get_gsc_board_id,
    get_gsc_version, get_kernel_rollback_version, get_tpm_version,
  },
};

/*
  Ideally we'd want to use the constants from Tlcl, but
  there is always a chance the user could either not want
  all of Tlcl, or they're on TPM 1.2 so they wouldn't be
  able to access TPM 2.0 constants.
*/
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
    "Specify a custom TPM device to use (/dev/tpmX format, e.g: /dev/tpm0",
  );
  let flags_disk_path = args.fequals_str(
    "--disk",
    "-d",
    "Specify a disk to read the GPT header from (/dev/[device] format, e.g: /dev/sda)",
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

  let disk_path = if flags_disk_path.is_empty() {
    "/dev/sda".to_string()
  } else {
    flags_disk_path.clone()
  };

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

  /* TODO(kxtz): make this easier to do with a sysinfo function */
  match File::open(&disk_path) {
    Ok(mut f) => match read_header(&mut f) {
      Ok(header) => {
        let sig = std::str::from_utf8(&header.magic).unwrap_or("???");
        let revision = header.revision;
        let header_size = header.header_size;
        let current_lba = header.current_lba;
        let backup_lba = header.backup_lba;
        let first_usable_lba = header.first_usable_lba;
        let last_usable_lba = header.last_usable_lba;
        let partition_entry_lba = header.partition_entry_lba;
        let num_entries = header.num_partition_entries;
        let entry_size = header.size_partition_entry;
        let header_crc32 = header.header_crc32;
        let part_crc32 = header.partition_entry_array_crc32;

        LOG!("GPT Header ({}):", disk_path);
        LOG!("  signature: {}", sig);
        LOG!("  revision: {}.{}", revision >> 16, revision & 0xffff);
        LOG!("  header size: {} bytes", header_size);
        LOG!("  current lba: {}", current_lba);
        LOG!("  backup lba: {}", backup_lba);
        LOG!(
          "  usable lba range: {} - {}",
          first_usable_lba,
          last_usable_lba
        );
        LOG!("  partition table @ lba: {}", partition_entry_lba);
        LOG!("  entries: {}", num_entries);
        LOG!("  entry size: {}", entry_size);
        LOG!("  header crc32: 0x{:08x}", header_crc32);
        LOG!("  partition array crc32: 0x{:08x}", part_crc32);
      }
      Err(e) => {
        LOG!("failed to read GPT header from {}: {}", disk_path, e);
      }
    },
    Err(e) => {
      LOG!("failed to open {}: {}", disk_path, e);
    }
  }

  /*
  TODO: we need the following:
  - emmc
  - cpu
  - ram
  - gpu(?)
  - battery (incl charge, estimated time remaining, etc)
  - ec info (version, chip, etc)
  - wp info
  - ap fw info (version, etc)
  - serial number
  */
}