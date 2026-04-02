use libcros::{LOG, Logger, kv_get, kv_set, libargs::ArgCheck, tlcl::TlclReadWithOffset};

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

  LOG!("reading 0xC bytes from index 0x1008 at offset 0x1 {}", tpm);
  let output = TlclReadWithOffset(0x1008, 0xC, 0x1);
  LOG!("read output: {:02X?}", output);
}
