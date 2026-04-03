pub mod constants;

use std::{
  fs::OpenOptions,
  io::{Read, Write},
  thread,
  time::Duration,
};

use crate::{keys, kv_get, LOG};

pub const TPM_MAX_RETRIES: u32 = 5;
pub const TPM_RETRY_DELAY_MS: u64 = 100;

pub fn tpm_xmit(
  sendbuf: *const u8,
  send_size: usize,
  recvbuf: *mut u8,
  recv_len: *mut usize,
) -> u32 {
  let tpm_path = kv_get(keys::TPM_PATH);
  let send_data = unsafe { core::slice::from_raw_parts(sendbuf, send_size) };

  let mut tpm_file = {
    let mut opened = None;
    for attempt in 0..TPM_MAX_RETRIES {
      match OpenOptions::new().read(true).write(true).open(&tpm_path) {
        Ok(file) => {
          opened = Some(file);
          break;
        }
        Err(_) => {
          if attempt + 1 < TPM_MAX_RETRIES {
            thread::sleep(Duration::from_millis(TPM_RETRY_DELAY_MS));
          }
        }
      }
    }

    match opened {
      Some(file) => file,
      None => return constants::TPM_E_COMMUNICATION_ERROR,
    }
  };

  if tpm_file.write_all(send_data).is_err() {
    return constants::TPM_E_COMMUNICATION_ERROR;
  }

  if !recvbuf.is_null() && !recv_len.is_null() {
    let recv_cap = unsafe { *recv_len };
    let recv_data = unsafe { core::slice::from_raw_parts_mut(recvbuf, recv_cap) };
    let received = match tpm_file.read(recv_data) {
      Ok(n) => n,
      Err(_) => return constants::TPM_E_COMMUNICATION_ERROR,
    };
    unsafe {
      *recv_len = received;
    }
  }
  
  return constants::TPM_SUCCESS;
}

pub fn vb2ex_tpm_send_recv(
  request: *const u8,
  request_length: u32,
  response: *mut u8,
  response_length: *mut u32,
) -> u32 {
  let mut len: usize = unsafe { *response_length } as usize;

  if tpm_xmit(request, request_length as usize, response, &mut len) != constants::TPM_SUCCESS {
    return constants::TPM_E_COMMUNICATION_ERROR;
  }

  if len > unsafe { *response_length } as usize {
    return constants::TPM_E_RESPONSE_TOO_LARGE;
  }

  unsafe {
    *response_length = len as u32;
  }
  constants::TPM_SUCCESS
}

pub mod commands;
pub mod permissions;

#[cfg(feature = "tpm1_2")]
pub mod tpm12;

#[cfg(feature = "tpm2_0")]
pub mod tpm20;

pub use commands::*;
pub use permissions::*;

// const TPM_MAX_RETRIES: u32 = 5;
// const TPM_RETRY_DELAY_MS: u64 = 100;
