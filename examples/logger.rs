use libcros::{LOG, LOG_DBG, LOG_FATAL, LOG_FATAL_NOEXIT, Logger};

fn main() {
  /*  arg1 to init is a boolean on whether or
      not we should be enabling verbose logging.
      verbose logging is done with LOG_DBG!(msg).

      arg2 is a boolean on whether or not not we
      should be using colors when logging. by default,
      we don't use colors but this can optionally be enabled.
  */
  Logger::init(true, true);

  LOG!("This is an regular log");
  LOG_DBG!("This is a verbose log");
  LOG_FATAL_NOEXIT!("This is a fatal log but will not automatically exit the program.");
  LOG_FATAL!("This is a fatal log and will automatically exit the program.");
}
