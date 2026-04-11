#![allow(non_snake_case)]

fn no_such_command() -> u32 {
  crate::tlcl::constants::TPM_E_NO_SUCH_COMMAND
}

pub fn TlclForceClear() -> u32 {
  no_such_command()
}

pub fn TlclDefineSpace(_index: u32, _perm: u32, _size: u32) -> u32 {
  no_such_command()
}

pub fn TlclUndefineSpace(_index: u32) -> u32 {
  no_such_command()
}

pub fn TlclUndefineSpaceEx(_owner_auth: *const u8, _owner_auth_size: u32, _index: u32) -> u32 {
  no_such_command()
}

pub fn TlclDefineSpaceEx(
  _owner_auth: *const u8,
  _owner_auth_size: u32,
  _index: u32,
  _perm: u32,
  _size: u32,
  _auth_policy: *const core::ffi::c_void,
  _auth_policy_size: u32,
) -> u32 {
  no_such_command()
}

pub fn TlclInitNvAuthPolicy(
  _pcr_selection_bitmap: u32,
  _pcr_values: *const u8,
  _auth_policy: *mut core::ffi::c_void,
  _auth_policy_size: &mut u32,
) -> u32 {
  no_such_command()
}

pub fn TlclGetSpaceInfo(
  _index: u32,
  _attributes: &mut u32,
  _size: &mut u32,
  _auth_policy: *mut core::ffi::c_void,
  _auth_policy_size: &mut u32,
) -> u32 {
  no_such_command()
}

pub fn TlclGetPermissions(_index: u32, _permissions: &mut u32) -> u32 {
  no_such_command()
}

pub fn TlclPhysicalPresenceCMDEnable() -> u32 {
  no_such_command()
}

pub fn TlclAssertPhysicalPresence() -> u32 {
  no_such_command()
}

pub fn TlclSetEnable() -> u32 {
  no_such_command()
}

pub fn TlclRead(_index: u32, _outbuf: *mut core::ffi::c_void, _length: u32) -> u32 {
  no_such_command()
}

pub fn TlclReadWithOffset(
  _index: u32,
  _size: u32,
  _offset: u32,
  _outbuf: *mut core::ffi::c_void,
) -> u32 {
  no_such_command()
}

pub fn TlclNVReadPublic(_index: u32, _presp: *mut core::ffi::c_void) -> u32 {
  no_such_command()
}

pub fn TlclStartup() -> u32 {
  no_such_command()
}

pub fn TlclSaveState() -> u32 {
  no_such_command()
}

pub fn TlclResume() -> u32 {
  no_such_command()
}

pub fn TlclWrite(_index: u32, _data: *const core::ffi::c_void, _length: u32) -> u32 {
  no_such_command()
}

pub fn TlclWriteWithOffset(
  _index: u32,
  _data: *const core::ffi::c_void,
  _length: u32,
  _offset: u32,
) -> u32 {
  no_such_command()
}
