use libcros::{LOG, Logger, kv_set, tlcl::TlclWrite};

fn main() {
  Logger::init(true, true);
  let tpm = "/dev/tpm1";
  kv_set(libcros::keys::TPM_PATH, tpm);

  LOG!("writing 0xD bytes to index 0x1008 {}", tpm);
  // kernver 0x00010002
  let output = TlclWrite(
    0x1008,
    &[
      0x02, 0x4C, 0x57, 0x52, 0x47, 0x02, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x33,
    ],
  );
  LOG!("write output: {:02X?}", output);
}
