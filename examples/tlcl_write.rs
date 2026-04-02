use libcros::{LOG, Logger, kv_get, kv_set, libargs::ArgCheck, tlcl::TlclWrite};

fn main() {
  let mut args: ArgCheck = ArgCheck::new();
  let verbose: bool = args.fbool("--verbose", "", "Enable debug messages");
  let flags_tpm_path = args.fequals_str(
    "--tpm-path",
    "-t",
    "Specify a custom TPM device to use in /dev/tpmX format",
  );

  args.check_help();

  if flags_tpm_path.is_empty() {
    kv_set(libcros::keys::TPM_PATH, "/dev/tpm69");
  } else {
    kv_set(libcros::keys::TPM_PATH, &flags_tpm_path);
  }

  Logger::init(verbose, true);
  let tpm = kv_get(libcros::keys::TPM_PATH);

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
