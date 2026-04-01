#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_unsafe)]
#![allow(dead_code)]
use std::{
  fmt,
  fs::OpenOptions,
  io::{self, Read, Write},
  mem,
  mem::ManuallyDrop,
  os::unix::io::AsRawFd,
  thread,
  time::Duration,
};

use crate::{LOG_DBG, keys, kv_get};

/*
Sources:
https://chromium.googlesource.com/chromiumos/platform/vboot_reference/+/e388d1f93c9573a79a04b633c3a0569ddbce6c94/firmware/include/tpm2_tss_constants.h
https://chromium.googlesource.com/chromiumos/third_party/tpm2/+/e2282558e2e1ffc992078f457539108eff1af246/tpm_types.h
*/
const TPM_ST_NO_SESSIONS: u16 = 0x8001;
const TPM_ST_SESSIONS: u16 = 0x8002;

const TPM_RS_PW: u32 = 0x40000009;
const TPM_RH_PLATFORM: u32 = 0x4000000C;

const TPM2_Clear: u32 = 0x126;
const TPM2_NV_Read: u32 = 0x14e;

const TPM_RC_SUCCESS: u32 = 0x000;
const TPM_RC_HANDLE: u32 = 0x18B; // RC_VER1 = 0x100 && RC_FMT1 = 0x80 && 0x18B = RC_VER1 + RC_FMT1 + 0xB
const TPM_RC_RETRY: u32 = 0x922; // RC_WARN = 0x900 && 0x922 = RC_WARN + 0x22

// This isn't from ChromiumOS, this is just options so we can control busy handling
const TPM_MAX_RETRIES: u32 = 5;
const TPM_RETRY_DELAY_MS: u64 = 100;

#[derive(Debug)]
struct Tpm2NvReadCmd {
  nvIndex: u32,
  size: u16,
  offset: u16,
}

#[derive(Debug)]
struct Tpm2SessionHeader {
  session_handle: u32,
  nonce_size: u16,
  nonce: u8,
  session_attrs: u8,
  auth_size: u16,
  auth: u8,
}

// tpm2_response start
#[derive(Debug)]
struct TPM2B {
  size: u16,
  buffer: *const u8,
}

#[derive(Debug)]
struct TpmsTaggedProperty {
  property: u32,
  value: u32,
}

#[derive(Debug)]
struct TpmlTaggedTpmProperty {
  count: u32,
  tpm_property: [TpmsTaggedProperty; 1],
}

union TpmuCapabilities {
  tpm_properties: ManuallyDrop<TpmlTaggedTpmProperty>,
}

impl fmt::Debug for TpmuCapabilities {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    unsafe {
      f.debug_struct("TpmuCapabilities")
        .field("tpm_properties", &self.tpm_properties)
        .finish()
    }
  }
}

#[derive(Debug)]
struct TpmsCapabilityData {
  capability: u32,
  data: TpmuCapabilities,
}

#[derive(Debug)]
struct NvReadResponse {
  params_size: u32,
  buffer: TPM2B,
}

#[derive(Debug)]
struct Tpm2TpmHeader {
  tpm_tag: u16,
  tpm_size: u32,
  tpm_code: u32,
}

#[derive(Debug)]
struct GetCapabilityResponse {
  more_data: u8,
  capability_data: TpmsCapabilityData,
}

#[derive(Debug)]
struct GetRandomResponse {
  random_bytes: TPM2B,
}

#[derive(Debug)]
struct TpmsNvPublic {
  nvIndex: u32,
  nameAlg: u16,
  attributes: u32,
  authPolicy: TPM2B,
  dataSize: u16,
}

#[derive(Debug)]
struct NvReadPublicResponse {
  nvPublic: TpmsNvPublic,
  nvName: TPM2B,
}

#[derive(Debug)]
struct ReadPublicResponse {
  buffer: TPM2B,
}

#[derive(Debug)]
struct CreatePrimaryResponse {
  object_handle: u32,
}

union Tpm2ResponseData {
  nvr: ManuallyDrop<NvReadResponse>,
  def_space: ManuallyDrop<Tpm2SessionHeader>,
  cap: ManuallyDrop<GetCapabilityResponse>,
  random: ManuallyDrop<GetRandomResponse>,
  nv_read_public: ManuallyDrop<NvReadPublicResponse>,
  read_pub: ManuallyDrop<ReadPublicResponse>,
  create_primary: ManuallyDrop<CreatePrimaryResponse>,
}

impl fmt::Debug for Tpm2ResponseData {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_struct("Tpm2ResponseData")
      .field("nvr", unsafe { &self.nvr })
      .field("def_space", unsafe { &self.def_space })
      .field("cap", unsafe { &self.cap })
      .field("random", unsafe { &self.random })
      .field("nv_read_public", unsafe { &self.nv_read_public })
      .field("read_pub", unsafe { &self.read_pub })
      .field("create_primary", unsafe { &self.create_primary })
      .finish()
  }
}

#[derive(Debug)]
struct Tpm2Response {
  hdr: Tpm2TpmHeader,
  data: Tpm2ResponseData,
}
// tpm2_response end

fn marshal_u32(value: u32, offset: &mut usize, array: &mut [u8; 512], size: &mut usize) {
  array[*offset] = (value >> 24) as u8;
  array[*offset + 1] = (value >> 16) as u8;
  array[*offset + 2] = (value >> 8) as u8;
  array[*offset + 3] = (value & 0xFF) as u8;
  *offset += 4;
  *size -= std::mem::size_of_val(&value);
}

fn marshal_u16(value: u16, offset: &mut usize, array: &mut [u8; 512], size: &mut usize) {
  array[*offset] = (value >> 8) as u8;
  array[*offset + 1] = (value & 0xFF) as u8;
  *offset += 2;
  *size -= std::mem::size_of_val(&value);
}

fn marshal_u8(value: u8, offset: &mut usize, array: &mut [u8; 512], size: &mut usize) {
  array[*offset] = value;
  *offset += 1;
  *size -= std::mem::size_of_val(&value);
}

fn unmarshal_u32(array: &[u8; 4096], offset: &mut usize) -> u32 {
  unsafe {
    let value = ((array[*offset] as u32) << 24)
      | ((array[*offset + 1] as u32) << 16)
      | ((array[*offset + 2] as u32) << 8)
      | (array[*offset + 3] as u32);
    *offset += 4;
    value
  }
}

fn unmarshal_u16(array: &[u8; 4096], offset: &mut usize) -> u16 {
  unsafe {
    let value = ((array[*offset] as u16) << 8) | (array[*offset + 1] as u16);
    *offset += 2;
    value
  }
}

fn unmarshal_u8(array: &[u8; 4096], offset: &mut usize) -> u8 {
  unsafe {
    let value = array[*offset];
    *offset += 1;
    value
  }
}
fn marshal_session_header_xptr(
  seshead: Tpm2SessionHeader,
  buffer: &mut [u8; 512],
  buf_size: &mut usize,
  offset: &mut usize,
) {
  let mut field_size_offset = *offset; // bypass for the offset.
  *offset += 4;
  let off_bef_session_head = *offset;
  marshal_u32(seshead.session_handle, &mut *offset, buffer, &mut *buf_size); // session handle
  marshal_u16(seshead.nonce_size, &mut *offset, buffer, &mut *buf_size); // nonce size
  marshal_u8(seshead.session_attrs, &mut *offset, buffer, &mut *buf_size); // session attrs
  marshal_u16(seshead.auth_size, &mut *offset, buffer, &mut *buf_size); // auth size
  let off_aft_session_head = *offset;

  marshal_u32(
    (off_aft_session_head as u32) - (off_bef_session_head as u32),
    &mut field_size_offset,
    buffer,
    &mut *buf_size,
  ); // field_size marshal
}

fn marshal_tpm_header(
  tpmhead: Tpm2TpmHeader,
  buffer: &mut [u8; 512],
  maxsize: &mut usize,
  offset: &mut usize,
) {
  marshal_u16(tpmhead.tpm_tag, &mut *offset, buffer, &mut *maxsize); // tpm_tag
  marshal_u32(tpmhead.tpm_size, &mut *offset, buffer, &mut *maxsize); // body_size
  marshal_u32(tpmhead.tpm_code, &mut *offset, buffer, &mut *maxsize); // command
}

fn open_tpm_device() -> io::Result<std::fs::File> {
  OpenOptions::new()
    .read(true)
    .write(true)
    .open(&kv_get(keys::TPM_PATH))
}

fn prep_marshal_readcmd(index: u32, size: u16, offset: u16) -> Tpm2NvReadCmd {
  LOG_DBG!(
    "index is 0x{:x}, size is 0x{:x}, offset is 0x{:x}",
    index,
    size,
    offset
  );
  let cmd = Tpm2NvReadCmd {
    nvIndex: 0x01000000 + index, // 0x01001008
    size: size,
    offset: offset,
  };
  cmd
}

fn marshal_readcmd(cmdbody: Tpm2NvReadCmd, buffer: &mut [u8; 512]) {
  // skip first 10 members, which is 0-9
  // we assume platform hierachy is disabled.
  let session_header_size = 10;
  let mut offset = session_header_size;
  let mut max_buf_size = 512 - session_header_size; // hardcoded sizes
  let mut buf_size = max_buf_size;
  marshal_u32(cmdbody.nvIndex, &mut offset, buffer, &mut buf_size); // index
  marshal_u32(cmdbody.nvIndex, &mut offset, buffer, &mut buf_size); // double index because thats what it expects if platform hierachy is disabled.

  let session_header = Tpm2SessionHeader {
    session_handle: TPM_RS_PW,
    nonce_size: 0,
    nonce: 0,
    session_attrs: 0,
    auth_size: 0,
    auth: 0,
  };
  marshal_session_header_xptr(session_header, buffer, &mut buf_size, &mut offset);

  marshal_u16(cmdbody.size, &mut offset, buffer, &mut buf_size); // size
  marshal_u16(cmdbody.offset, &mut offset, buffer, &mut buf_size); // offset

  offset = 0; // we fill in tpm_header now
  let calc_buf_size: u32 = (max_buf_size as u32) - (buf_size as u32) + 10; // tpm_header size
  let tpm_header = Tpm2TpmHeader {
    tpm_tag: TPM_ST_SESSIONS,
    tpm_size: calc_buf_size,
    tpm_code: TPM2_NV_Read,
  };
  marshal_tpm_header(tpm_header, buffer, &mut max_buf_size, &mut offset);
}

fn marshal_clear_cmd(buffer: &mut [u8; 512]) {
  let session_header_size = 10;
  let mut offset = session_header_size;
  let mut max_buf_size = 512 - session_header_size;
  let mut buf_size = max_buf_size;

  marshal_u32(TPM_RH_PLATFORM, &mut offset, buffer, &mut buf_size);

  let session_header = Tpm2SessionHeader {
    session_handle: TPM_RS_PW,
    nonce_size: 0,
    nonce: 0,
    session_attrs: 0,
    auth_size: 0,
    auth: 0,
  };
  marshal_session_header_xptr(session_header, buffer, &mut buf_size, &mut offset);

  offset = 0;
  let calc_buf_size: u32 = (max_buf_size as u32) - (buf_size as u32) + 10;
  let tpm_header = Tpm2TpmHeader {
    tpm_tag: TPM_ST_SESSIONS,
    tpm_size: calc_buf_size,
    tpm_code: TPM2_Clear,
  };
  marshal_tpm_header(tpm_header, buffer, &mut max_buf_size, &mut offset);
}

fn unmarshal_readcmd(buffer: &mut [u8; 4096], size: u16, response: &mut Tpm2Response) -> Vec<u8> {
  unsafe {
    let mut offset = 0;
    response.hdr.tpm_tag = unmarshal_u16(buffer, &mut offset);
    response.hdr.tpm_size = unmarshal_u32(buffer, &mut offset);
    response.hdr.tpm_code = unmarshal_u32(buffer, &mut offset);
    (&mut (*(&mut response.data.nvr as *mut ManuallyDrop<NvReadResponse>))).params_size =
      unmarshal_u32(buffer, &mut offset);
    (&mut (*(&mut response.data.nvr as *mut ManuallyDrop<NvReadResponse>)))
      .buffer
      .size = unmarshal_u16(buffer, &mut offset);

    // final bytes
    let end = offset + (size as usize);
    let mut bytes = Vec::with_capacity(size as usize);
    bytes.extend_from_slice(&buffer[offset..end]);
    bytes
  }
}

fn unmarshal_response_code(buffer: &[u8; 4096]) -> u32 {
  let mut offset = 0;
  let _tag = unmarshal_u16(buffer, &mut offset);
  let _size = unmarshal_u32(buffer, &mut offset);
  unmarshal_u32(buffer, &mut offset) // return code
}

/*
Typically, Chrome devices have stuff like tcsd or trunksd which
keep tpm0 in use and will make the program think we're returning
all zeros from the TPM when unmarshaling.

Use this function so that we can properly handle if tpm0 is in use or not.
*/
fn tpm_exchange(tin: &mut [u8; 512], tout: &mut [u8; 4096]) -> io::Result<()> {
  let mut retries = 0;

  loop {
    let mut tpm_file = match open_tpm_device() {
      Ok(file) => file,
      Err(e) => {
        if (e.kind() == io::ErrorKind::PermissionDenied
          || e.kind() == io::ErrorKind::WouldBlock
          || e.raw_os_error() == Some(16))
          && retries < TPM_MAX_RETRIES
        {
          LOG_DBG!(
            "TPM device busy or permission denied, retrying ({}/{})",
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

    LOG_DBG!("Using fd {}", tpm_file.as_raw_fd());

    tpm_file.write_all(tin)?;

    let bytes_read = match tpm_file.read(tout) {
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
      bytes_read,
      tout[..bytes_read]
        .iter()
        .map(|b| format!("{:x}", b))
        .collect::<Vec<String>>()
    );

    let response_code = unmarshal_response_code(tout);
    LOG_DBG!("tpm rc: 0x{:x}", response_code);
    if response_code == TPM_RC_HANDLE {
      LOG_DBG!("NVIndex doesn't exist! (the handle is not correct for the use)");
      return Err(io::Error::new(
        io::ErrorKind::NotFound,
        "TPM NV index does not exist",
      ));
    }

    if response_code == TPM_RC_RETRY && retries < TPM_MAX_RETRIES {
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

pub fn TlclRead(index: u32, size: u16) -> Vec<u8> {
  TlclReadWithOffset(index, size, 0)
}

pub fn TlclReadWithOffset(index: u32, size: u16, offset: u16) -> Vec<u8> {
  let mut cr_buffer: [u8; 512] = [0; 512]; // size is 512, we will hardcode this
  let mut response_buffer: [u8; 4096] = [0; 4096];

  // marshal :explode:
  let cmd = prep_marshal_readcmd(index, size, offset);
  marshal_readcmd(cmd, &mut cr_buffer);

  match tpm_exchange(&mut cr_buffer, &mut response_buffer) {
    Ok(_) => {
      // unmarshal
      let mut response: Tpm2Response = unsafe { mem::zeroed() };
      let final_bytes = unmarshal_readcmd(&mut response_buffer, size, &mut response);
      final_bytes
    }
    Err(e) => {
      LOG_DBG!("ERROR: TPM exchange failed: {}", e);
      Vec::new()
    }
  }
}

// pub fn TlclWrite(index: u32, data: *const c_void, size: u16) -> Vec<u8> {
//   LOG_DBG!("Not implemented. Index: {:x}, Data: {:?}, Size: {:x}", index, data, size);
//   Vec::new()
// }

pub fn TlclForceClear() -> u32 {
  let mut cr_buffer: [u8; 512] = [0; 512];
  let mut response_buffer: [u8; 4096] = [0; 4096];

  marshal_clear_cmd(&mut cr_buffer);

  match tpm_exchange(&mut cr_buffer, &mut response_buffer) {
    Ok(_) => {
      let response_code = unmarshal_response_code(&response_buffer);
      LOG_DBG!("TlclForceClear response code: 0x{:x}", response_code);
      response_code
    }
    Err(e) => {
      LOG_DBG!("ERROR: TPM exchange failed in TlclForceClear: {}", e);
      0xFFFFFFFF
    }
  }
}
