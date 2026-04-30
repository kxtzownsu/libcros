use std::fs::File;

use libcros::diskutils::gpt::{GptDisk, read_entries, read_header};
use libcros::libargs::ArgCheck;

fn main() -> std::io::Result<()> {
  let mut args = ArgCheck::new();
  let flags_disk = args.fequals_str("--disk", "-d", "Which disk to use");
  let flags_partition_id = args.fequals_str("--id", "-p", "Which partition ID to use");

  args.check_help();

  let mut disk_path = flags_disk.clone();
  let mut partition_id: u32 = flags_partition_id.parse().unwrap_or(12);

  if flags_disk.is_empty() {
    disk_path = "/dev/sda".to_string();
  }

  let mut disk_file = File::open(disk_path)?;
  let header = read_header(&mut disk_file)?;
  let entries = read_entries(&mut disk_file, &header)?;
  let gpt = GptDisk { header, entries };

  match gpt.get_partition_type(partition_id) {
    Some(uuid) => {
      let b = uuid.as_bytes();
      println!(
        "{:08x}-{:04x}-{:04x}-{:04x}-{:012x}",
        u32::from_be_bytes(b[0..4].try_into().unwrap()),
        u16::from_be_bytes(b[4..6].try_into().unwrap()),
        u16::from_be_bytes(b[6..8].try_into().unwrap()),
        u16::from_be_bytes(b[8..10].try_into().unwrap()),
        {
          let mut tail = [0u8; 8];
          tail[2..].copy_from_slice(&b[10..16]);
          u64::from_be_bytes(tail)
        }
      );
    }
    None => eprintln!("partition 12 not found or empty"),
  }

  Ok(())
}