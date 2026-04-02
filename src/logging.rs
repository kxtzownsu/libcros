static mut VERBOSE: bool = false;

pub struct Logger;

// use_colors isn't implemented at the moment, ignore it.
impl Logger {
  pub fn init(verbose: bool, _use_colors: bool) {
    unsafe {
      VERBOSE = verbose;
    }
  }

  pub fn verbose_enabled() -> bool {
    unsafe { VERBOSE }
  }
}

#[macro_export]
macro_rules! LOG {
  ($($arg:tt)*) => {
    println!(
      "INFO [{}:{}]: {}",
      file!(),
      line!(),
      format_args!($($arg)*)
    );
  };
}

#[macro_export]
macro_rules! LOG_DBG {
  ($($arg:tt)*) => {
    if $crate::Logger::verbose_enabled() {
      println!(
        "DEBUG [{}:{}]: {}",
        file!(),
        line!(),
        format_args!($($arg)*)
      );
    }
  };
}

#[macro_export]
macro_rules! LOG_FATAL {
  // LOG_FATAL!(67; "error"), would exit with rc 67
  ($rc:expr; $($arg:tt)*) => {{
    println!(
      "FATAL [{}:{}]: {}",
      file!(),
      line!(),
      format_args!($($arg)*)
    );
    std::process::exit($rc);
  }};
  // LOG_FATAL!("error"), would exit with rc -1
  ($($arg:tt)*) => {{
    println!(
      "FATAL [{}:{}]: {}",
      file!(),
      line!(),
      format_args!($($arg)*)
    );
    std::process::exit(-1);
  }};
}

#[macro_export]
macro_rules! LOG_FATAL_NOEXIT {
  // LOG_FATAL_NOEXIT!(42; "error"), sicne we dont exit, the rc is ignored.
  ($_rc:expr; $($arg:tt)*) => {
    println!(
      "FATAL [{}:{}]: {}",
      file!(),
      line!(),
      format_args!($($arg)*)
    );
  };
  // LOG_FATAL_NOEXIT!("error"), same thign, rc is ignored.
  ($($arg:tt)*) => {
    println!(
      "FATAL [{}:{}]: {}",
      file!(),
      line!(),
      format_args!($($arg)*)
    );
  };
}
