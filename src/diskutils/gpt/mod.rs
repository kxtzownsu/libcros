pub mod disk; pub use disk::*;
pub mod partitions;

pub const GUID_LINUX_FILESYSTEM: Uuid = Uuid([
  0x0F, 0xC6, 0x3D, 0xAF, 0x84, 0x83, 0x47, 0x72,
  0x8E, 0x79, 0x3D, 0x69, 0xD8, 0x47, 0x7D, 0xE4,
]); /* 0FC63DAF-8483-4772-8E79-3D69D8477DE4 */

pub const GUID_EFI_SYSTEM_PARTITION: Uuid = Uuid([
  0xC1, 0x2A, 0x73, 0x28, 0xF8, 0x1F, 0x11, 0xD2,
  0xBA, 0x4B, 0x00, 0xA0, 0xC9, 0x3E, 0xC9, 0x3B,
]); /* C12A7328-F81F-11D2-BA4B-00A0C93EC93B */

pub const GUID_CHROMEOS_KERNEL: Uuid = Uuid([
  0xFE, 0x3A, 0x2A, 0x5D, 0x4F, 0x32, 0x41, 0xA7,
  0xB7, 0x25, 0xAC, 0xCC, 0x32, 0x85, 0xA3, 0x09,
]); /* FE3A2A5D-4F32-41A7-B725-ACCC3285A309 */

pub const GUID_CHROMEOS_ROOTFS: Uuid = Uuid([
  0x3C, 0xB8, 0xE2, 0x02, 0x3B, 0x7E, 0x47, 0xDD,
  0x8A, 0x3C, 0x7F, 0xF2, 0xA1, 0x3C, 0xFC, 0xEC,
]); /* 3CB8E202-3B7E-47DD-8A3C-7FF2A13CFCEC */


#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Uuid([u8; 16]);

impl Uuid {
  pub fn from_bytes(bytes: [u8; 16]) -> Self {
    Self(bytes)
  }

  pub fn as_bytes(&self) -> &[u8; 16] {
    &self.0
  }
}

pub fn gpt_guid_to_uuid(raw: &[u8; 16]) -> Uuid {
  let mut bytes = *raw;
  bytes[0..4].reverse();
  bytes[4..6].reverse();
  bytes[6..8].reverse();
  Uuid::from_bytes(bytes)
}

pub fn uuid_to_gpt_guid(uuid: &Uuid) -> [u8; 16] {
  let mut bytes = *uuid.as_bytes();
  bytes[0..4].reverse();
  bytes[4..6].reverse();
  bytes[6..8].reverse();
  bytes
}
