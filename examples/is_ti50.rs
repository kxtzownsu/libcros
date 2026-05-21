use libcros::{
  LOG_FATAL, Logger,
  gsc::{
    constants::{VENDOR_RC_NO_SUCH_COMMAND, vendor_cmd_cc::VENDOR_CC_GET_TI50_STATS},
    read_response, send_command, tpm20,
  },
  key_types,
  kv_get,
  libargs::ArgCheck,
  sysinfo::backend::{close_gsc_socket, open_gsc_socket},
};

fn main() {
  let mut args: ArgCheck = ArgCheck::new();
  let verbose: bool = args.fbool("--verbose", "", "Enable debug messages");

  args.check_help();
  Logger::init(verbose, true);

  open_gsc_socket();

  let mut f = match kv_get(key_types::FILE, libcros::keys::GSC_SOCKET) {
    Some(libcros::KvValue::File(f)) => f,
    _ => LOG_FATAL!("failed to get GSC socket"),
  };

  if !send_command(&mut f, 0, 0, &[], VENDOR_CC_GET_TI50_STATS) {
    close_gsc_socket();
    LOG_FATAL!("failed to send GET_TI50_STATS");
  }

  let mut buf = [];
  let rc = read_response(&mut f, &mut buf);

  close_gsc_socket();

  let chip = if rc == tpm20::types::TPM_RC_BAD_TAG {
    "TPM 1.2"
  } else if rc == VENDOR_RC_NO_SUCH_COMMAND {
    "Cr50"
  } else {
    "Ti50"
  };

  println!("{}", chip);
}
