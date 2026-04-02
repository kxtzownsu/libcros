use libcros::{LOG, Logger, kv_set, tlcl::TlclReadWithOffset};

fn main() {
  Logger::init(true, true);
  let tpm = "/dev/tpm1";
  kv_set(libcros::keys::TPM_PATH, tpm);

  LOG!("reading 0xC bytes from index 0x1008 at offset 0x1 {}", tpm);
  let output = TlclReadWithOffset(0x1008, 0xC, 0x1);
  LOG!("read output: {:02X?}", output);
}
