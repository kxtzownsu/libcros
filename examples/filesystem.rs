use std::fs::File;

use libcros::{
  LOG, LOG_FATAL, Logger,
  libargs::ArgCheck,
  ui::utils::input,
  diskutils::gpt::{read_header, read_entries, GptDisk},
  fs::{filesystems, verify_filesystem}
};

fn main() {
  let mut args = ArgCheck::new();
  let verbose: bool = args.fbool("--verbose", "", "Enable debug messages");
  let flags_disk_path = args.fequals_str(
    "--disk",
    "-d",
    "Specify a disk to read the GPT header from (/dev/[device] format, e.g: /dev/sda)",
  );

  args.check_help();
  Logger::init(verbose, true);

  if flags_disk_path.is_empty() {
    LOG_FATAL!("disk path not specified");
  }

  let disk_path = flags_disk_path.clone();

  LOG!("Before continuing, this example assumes there is an ext2/3/4 filesystem on partition 1, a fat16 filesystem on partition 2, and a fat32 filesystem on partition 3");

  let mut response = input("Are you sure you want to continue? (y/N) ");
  response = response.to_lowercase();

  match response.as_str() {
    "y" | "yes" => {}
    _ => LOG_FATAL!("exiting!")
  }

  let mut f = match File::open(&disk_path) {
    Ok(f) => f,
    Err(e) => LOG_FATAL!("failed to open {}: {}", disk_path, e),
  };

  let header = match read_header(&mut f) {
    Ok(h) => h,
    Err(e) => LOG_FATAL!("failed to read gpt header: {}", e),
  };

  let entries = match read_entries(&mut f, &header) {
    Ok(e) => e,
    Err(e) => LOG_FATAL!("failed to read gpt entries: {}", e),
  };

  let disk = GptDisk { header, entries };

  let mut check = |id: u32, fs: &str| {
    match disk.partition(id) {
      Some(p) => {
        let base = p.starting_lba * 512;

        if verify_filesystem(fs, &mut f, base) {
          LOG!("partition {}: {} (ok)", id, fs);
        } else {
          LOG!("partition {}: {} (failed)", id, fs);
        }
      }
      None => LOG!("partition {}: not present", id),
    }
  };

  check(1, filesystems::EXT2); // ext2/3/4 all share same verifier
  check(2, filesystems::FAT16);
  check(3, filesystems::FAT32);
}