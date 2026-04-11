use crate::tlcl::tpm20::constants::nv_read_public_response;

pub struct Client;

impl Client {
  pub fn new() -> Self {
    Self
  }

  pub fn force_clear(&self) -> u32 {
    crate::tlcl::commands::tpm20::TlclForceClear()
  }

  pub fn define_space(&self, index: u32, perm: u32, size: u32) -> u32 {
    crate::tlcl::commands::tpm20::TlclDefineSpace(index, perm, size)
  }

  pub fn undefine_space(&self, index: u32) -> u32 {
    crate::tlcl::commands::tpm20::TlclUndefineSpace(index)
  }

  pub fn undefine_space_ex(&self, owner_auth: *const u8, owner_auth_size: u32, index: u32) -> u32 {
    crate::tlcl::commands::tpm20::TlclUndefineSpaceEx(owner_auth, owner_auth_size, index)
  }

  pub fn define_space_ex(
    &self,
    owner_auth: *const u8,
    owner_auth_size: u32,
    index: u32,
    perm: u32,
    size: u32,
    auth_policy: *const core::ffi::c_void,
    auth_policy_size: u32,
  ) -> u32 {
    crate::tlcl::commands::tpm20::TlclDefineSpaceEx(
      owner_auth,
      owner_auth_size,
      index,
      perm,
      size,
      auth_policy,
      auth_policy_size,
    )
  }

  pub fn init_nv_auth_policy(
    &self,
    pcr_selection_bitmap: u32,
    pcr_values: *const u8,
    auth_policy: *mut core::ffi::c_void,
    auth_policy_size: &mut u32,
  ) -> u32 {
    crate::tlcl::commands::tpm20::TlclInitNvAuthPolicy(
      pcr_selection_bitmap,
      pcr_values,
      auth_policy,
      auth_policy_size,
    )
  }

  pub fn get_space_info(
    &self,
    index: u32,
    attributes: &mut u32,
    size: &mut u32,
    auth_policy: *mut core::ffi::c_void,
    auth_policy_size: &mut u32,
  ) -> u32 {
    crate::tlcl::commands::tpm20::TlclGetSpaceInfo(
      index,
      attributes,
      size,
      auth_policy,
      auth_policy_size,
    )
  }

  pub fn get_permissions(&self, index: u32, permissions: &mut u32) -> u32 {
    crate::tlcl::commands::tpm20::TlclGetPermissions(index, permissions)
  }

  pub fn physical_presence_cmd_enable(&self) -> u32 {
    crate::tlcl::commands::tpm20::TlclPhysicalPresenceCMDEnable()
  }

  pub fn assert_physical_presence(&self) -> u32 {
    crate::tlcl::commands::tpm20::TlclAssertPhysicalPresence()
  }

  pub fn set_enable(&self) -> u32 {
    crate::tlcl::commands::tpm20::TlclSetEnable()
  }

  pub fn read(&self, index: u32, outbuf: *mut core::ffi::c_void, length: u32) -> u32 {
    crate::tlcl::commands::tpm20::TlclRead(index, outbuf, length)
  }

  pub fn read_with_offset(
    &self,
    index: u32,
    size: u32,
    offset: u32,
    outbuf: *mut core::ffi::c_void,
  ) -> u32 {
    crate::tlcl::commands::tpm20::TlclReadWithOffset(index, size, offset, outbuf)
  }

  pub fn nv_read_public(&self, index: u32, presp: *mut nv_read_public_response) -> u32 {
    crate::tlcl::commands::tpm20::TlclNVReadPublic(index, presp)
  }

  pub fn startup(&self) -> u32 {
    crate::tlcl::commands::tpm20::TlclStartup()
  }

  pub fn save_state(&self) -> u32 {
    crate::tlcl::commands::tpm20::TlclSaveState()
  }

  pub fn resume(&self) -> u32 {
    crate::tlcl::commands::tpm20::TlclResume()
  }

  pub fn write(&self, index: u32, data: *const core::ffi::c_void, length: u32) -> u32 {
    crate::tlcl::commands::tpm20::TlclWrite(index, data, length)
  }

  pub fn write_with_offset(
    &self,
    index: u32,
    data: *const core::ffi::c_void,
    length: u32,
    offset: u32,
  ) -> u32 {
    crate::tlcl::commands::tpm20::TlclWriteWithOffset(index, data, length, offset)
  }
}
