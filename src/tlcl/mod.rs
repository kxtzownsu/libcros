#![allow(non_snake_case)]

mod backend;
pub mod bytes;
mod client;
pub mod constants;

use std::{
  fs::OpenOptions,
  io::{Error, ErrorKind, Read, Write},
  thread,
  time::Duration,
};

use crate::{LOG_FATAL, keys, keyval::KvValue, kv_get};

pub const TPM_MAX_RETRIES: u32 = 5;
pub const TPM_RETRY_DELAY_MS: u64 = 100;

const TPM_HEADER_SIZE: usize = 10;

enum Response {
  Complete(usize),
  TooLarge(usize),
}

fn should_retry_io(err: &Error) -> bool {
  matches!(
    err.kind(),
    ErrorKind::PermissionDenied | ErrorKind::WouldBlock | ErrorKind::Interrupted
  ) || err.raw_os_error() == Some(16)
}

fn read_response<R: Read>(reader: &mut R, buf: &mut [u8]) -> Result<Response, Error> {
  let mut received = 0usize;

  while received < buf.len() {
    match reader.read(&mut buf[received..]) {
      Ok(0) => break,
      Ok(read) => {
        received += read;

        if received < TPM_HEADER_SIZE {
          continue;
        }

        let total_size = u32::from_be_bytes([buf[2], buf[3], buf[4], buf[5]]) as usize;

        if total_size < TPM_HEADER_SIZE {
          return Err(Error::new(ErrorKind::InvalidData, "short TPM response"));
        }
        if total_size > buf.len() {
          return Ok(Response::TooLarge(total_size));
        }
        if received >= total_size {
          return Ok(Response::Complete(total_size));
        }
      }
      Err(err) if err.kind() == ErrorKind::Interrupted => continue,
      Err(err) => return Err(err),
    }
  }

  Err(Error::new(
    ErrorKind::UnexpectedEof,
    "truncated TPM response",
  ))
}

pub fn tpm_xmit(
  sendbuf: *const u8,
  send_size: usize,
  recvbuf: *mut u8,
  recv_len: *mut usize,
) -> u32 {
  const TPM_HEADER_SIZE: usize = 10;

  let tpm_path = match kv_get("string", keys::TPM_PATH) {
    Some(KvValue::String(s)) => s,
    _ => LOG_FATAL!("TPM_PATH missing or wrong type!"),
  };

  if tpm_path.is_empty() {
    LOG_FATAL!("TPM_PATH not set! cannot continue!");
  }

  let send_data = unsafe { core::slice::from_raw_parts(sendbuf, send_size) };

  for attempt in 0..TPM_MAX_RETRIES {
    let mut tpm_file = match OpenOptions::new().read(true).write(true).open(&tpm_path) {
      Ok(file) => file,
      Err(err) if should_retry_io(&err) && attempt + 1 < TPM_MAX_RETRIES => {
        thread::sleep(Duration::from_millis(TPM_RETRY_DELAY_MS));
        continue;
      }
      Err(_) => return constants::TPM_E_COMMUNICATION_ERROR,
    };

    if let Err(err) = tpm_file.write_all(send_data) {
      if should_retry_io(&err) && attempt + 1 < TPM_MAX_RETRIES {
        thread::sleep(Duration::from_millis(TPM_RETRY_DELAY_MS));
        continue;
      }
      return constants::TPM_E_COMMUNICATION_ERROR;
    }

    if recvbuf.is_null() || recv_len.is_null() {
      return constants::TPM_SUCCESS;
    }

    let recv_cap = unsafe { *recv_len };
    if recv_cap < TPM_HEADER_SIZE {
      return constants::TPM_E_RESPONSE_TOO_LARGE;
    }

    let recv_data = unsafe { core::slice::from_raw_parts_mut(recvbuf, recv_cap) };

    let response = match read_response(&mut tpm_file, recv_data) {
      Ok(response) => response,
      Err(err) if should_retry_io(&err) && attempt + 1 < TPM_MAX_RETRIES => {
        thread::sleep(Duration::from_millis(TPM_RETRY_DELAY_MS));
        continue;
      }
      Err(_) => return constants::TPM_E_COMMUNICATION_ERROR,
    };

    match response {
      Response::TooLarge(total_size) => {
        unsafe {
          *recv_len = total_size;
        }
        return constants::TPM_E_RESPONSE_TOO_LARGE;
      }
      Response::Complete(total_size) => {
        unsafe {
          *recv_len = total_size;
        }
        return constants::TPM_SUCCESS;
      }
    }
  }

  constants::TPM_E_COMMUNICATION_ERROR
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
pub mod exports;
pub mod permissions;

#[cfg(feature = "tpm1_2")]
pub mod tpm12;

#[cfg(feature = "tpm2_0")]
pub mod tpm20;

#[cfg(feature = "tpm1_2")]
pub mod v1_2;

#[cfg(feature = "tpm2_0")]
pub mod v2_0;

#[cfg(all(not(feature = "tpm1_2"), not(feature = "tpm2_0")))]
pub mod stubs;

#[allow(unused_imports)]
pub use exports::*;
#[cfg(any(feature = "tpm1_2", feature = "tpm2_0"))]
#[allow(unused_imports)]
pub use permissions::*;

#[cfg(all(feature = "tpm2_0", feature = "tpm1_2"))]
pub fn TlclGetTPMVersion() -> String {
  "2.0".to_string()
}

#[cfg(all(feature = "tpm2_0", not(feature = "tpm1_2")))]
pub fn TlclGetTPMVersion() -> String {
  "2.0".to_string()
}

#[cfg(all(feature = "tpm1_2", not(feature = "tpm2_0")))]
pub fn TlclGetTPMVersion() -> String {
  "1.2".to_string()
}

#[cfg(not(any(feature = "tpm1_2", feature = "tpm2_0")))]
pub fn TlclGetTPMVersion() -> String {
  "No TPM version was specified when compiling libcros.".to_string()
}
