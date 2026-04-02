use libcros::{LOG, LOG_DBG, LOG_FATAL, LOG_FATAL_NOEXIT, Logger, libargs::ArgCheck};

fn main() {
  let mut args: ArgCheck = ArgCheck::new();
  let verbose: bool = args.fbool("--verbose", "", "Enable debug messages");
  let use_colors: bool = args.fbool("--colors", "-c", "Use colors when logging");
  let rc = args.fequals_str("--rc", "-r", "Exit code for fatal errors");
  args.check_help();

  let parsed_rc: i32 = if rc.is_empty() {
    -1
  } else {
    rc.parse().unwrap_or(-1)
  };

  /*  arg1 to init is a boolean on whether or
      not we should be enabling verbose logging.
      verbose logging is done with LOG_DBG!(msg).

      arg2 is a boolean on whether or not not we
      should be using colors when logging. by default,
      we don't use colors but this can optionally be enabled.
  */
  Logger::init(verbose, use_colors);

  LOG!("This is an regular log");
  LOG_DBG!("This is a verbose log");
  LOG_FATAL_NOEXIT!("This is a fatal log but will not automatically exit the program.");
  LOG_FATAL!(parsed_rc;
    "This is a fatal log and will automatically exit the program."
  );
}
