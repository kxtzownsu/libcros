/// Easy-to-use key=value system with types.
pub mod keyval;
pub use keyval::*;

/// Easy-to-use logging API
pub mod logging;
pub use logging::Logger;

/// Basic cryptography functions
pub mod crypto;

/// Easy-to-use functions to execute commands
pub mod execute;

/// Lightweight argument parser
pub mod libargs;

/// Commonly-used structs
pub mod structs;

/// High-level functions to get information about a Chrome device
pub mod sysinfo;

/// Send/receive vendor commands to/from the GSC (Google Security Chip) / TPM.
pub mod gsc;

/// High-level functions to interact with a GPT-formatted disk. (Including ChromeOS disks)
pub mod diskutils;

/*
Anything that requires a dependency should be locked behind a feature flag.
*/

/// Basic drawing functions for TUIs
#[cfg(feature = "ui")]
pub mod ui;

/// A lightweight TPM2 library. Based on vboot's Tlcl library.
#[cfg(feature = "tlcl")]
pub mod tlcl;

/*
pub mod vpd;
*/
