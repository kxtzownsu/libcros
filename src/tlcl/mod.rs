use std::{
  fs::OpenOptions,
  io::{self, Read, Write},
  os::unix::io::AsRawFd,
  thread,
  time::Duration,
};

use crate::{LOG_DBG, keys, kv_get};

pub mod commands;
pub mod permissions;
pub mod unmarshal;

#[cfg(feature = "tpm1_2")]
pub mod tpm12;

#[cfg(feature = "tpm2_0")]
pub mod tpm20;

#[cfg(feature = "tpm2_0")]
pub mod marshal;
#[cfg(feature = "tpm2_0")]
pub mod structs;

pub use commands::*;
pub use permissions::*;

const TPM_MAX_RETRIES: u32 = 5;
const TPM_RETRY_DELAY_MS: u64 = 100;

#[cfg(feature = "tpm2_0")]
const TPM_RESPONSE_RETRY: u32 = tpm20::constants::TPM_RC_RETRY;
#[cfg(feature = "tpm2_0")]
const TPM_RESPONSE_HANDLE_NOT_FOUND: u32 = tpm20::constants::TPM_RC_HANDLE;

#[cfg(feature = "tpm1_2")]
const TPM_RESPONSE_RETRY: u32 = tpm12::constants::TPM_E_RETRY;
#[cfg(feature = "tpm1_2")]
const TPM_RESPONSE_HANDLE_NOT_FOUND: u32 = tpm12::constants::TPM_E_BADINDEX;

fn open_tpm_device() -> io::Result<std::fs::File> {
  OpenOptions::new()
    .read(true)
    .write(true)
    .open(&kv_get(keys::TPM_PATH))
}

/*
Typically, Chrome devices have stuff like tcsd or trunksd which keep tpm0 in
use and will make the program think we're returning all zeros from the TPM
when unmarshaling. This handles busy/retry conditions before giving up.
*/
pub(crate) fn tpm_exchange(tin: &mut [u8; 512], tout: &mut [u8; 4096]) -> io::Result<()> {
  let mut retries = 0;

  loop {
    let mut tpm_file = match open_tpm_device() {
      Ok(f) => f,
      Err(e) => {
        if (e.kind() == io::ErrorKind::PermissionDenied
          || e.kind() == io::ErrorKind::WouldBlock
          || e.raw_os_error() == Some(16))
          && retries < TPM_MAX_RETRIES
        {
          LOG_DBG!(
            "TPM busy/permission denied, retrying ({}/{})",
            retries + 1,
            TPM_MAX_RETRIES
          );
          retries += 1;
          thread::sleep(Duration::from_millis(TPM_RETRY_DELAY_MS));
          continue;
        }
        return Err(e);
      }
    };

    tpm_file.write_all(tin)?;

    let n = match tpm_file.read(tout) {
      Ok(n) => n,
      Err(e) => {
        if e.kind() == io::ErrorKind::WouldBlock && retries < TPM_MAX_RETRIES {
          LOG_DBG!(
            "TPM read would block, retrying ({}/{})",
            retries + 1,
            TPM_MAX_RETRIES
          );
          retries += 1;
          thread::sleep(Duration::from_millis(TPM_RETRY_DELAY_MS));
          continue;
        }
        return Err(e);
      }
    };

    LOG_DBG!(
      "read {} bytes: {:?}",
      n,
      tout[..n]
        .iter()
        .map(|b| format!("{:x}", b))
        .collect::<Vec<_>>()
    );

    let rc = unmarshal::unmarshal_response_code(tout);
    LOG_DBG!("rc=0x{:x}", rc);

    if rc == TPM_RESPONSE_HANDLE_NOT_FOUND {
      return Err(io::Error::new(
        io::ErrorKind::NotFound,
        "TPM NV index does not exist",
      ));
    }

    if rc == TPM_RESPONSE_RETRY && retries < TPM_MAX_RETRIES {
      LOG_DBG!(
        "TPM returned RETRY, attempt {}/{}",
        retries + 1,
        TPM_MAX_RETRIES
      );
      retries += 1;
      thread::sleep(Duration::from_millis(TPM_RETRY_DELAY_MS));
      continue;
    }

    return Ok(());
  }
}
