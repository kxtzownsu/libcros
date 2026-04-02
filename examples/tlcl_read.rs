use libcros::{LOG, Logger, kv_set, tlcl::TlclRead};

fn main() {
  Logger::init(true, true);
  let tpm = "/dev/tpm1";
  kv_set(libcros::keys::TPM_PATH, tpm);

  LOG!("reading 0xD bytes from index 0x1008 {}", tpm);
  let output = TlclRead(0x1008, 0xD);
  LOG!("read output: {:02X?}", output);
}
