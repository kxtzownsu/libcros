macro_rules! DEFINE_TLCL_EXPORT {
  ($export_name:ident, $internal:path) => {
    pub use $internal as $export_name;
  };
}

DEFINE_TLCL_EXPORT!(TlclForceClear, crate::tlcl::commands::TlclForceClear);
DEFINE_TLCL_EXPORT!(TlclDefineSpace, crate::tlcl::commands::TlclDefineSpace);
DEFINE_TLCL_EXPORT!(TlclUndefineSpace, crate::tlcl::commands::TlclUndefineSpace);
DEFINE_TLCL_EXPORT!(
  TlclUndefineSpaceEx,
  crate::tlcl::commands::TlclUndefineSpaceEx
);
DEFINE_TLCL_EXPORT!(TlclDefineSpaceEx, crate::tlcl::commands::TlclDefineSpaceEx);
DEFINE_TLCL_EXPORT!(
  TlclInitNvAuthPolicy,
  crate::tlcl::commands::TlclInitNvAuthPolicy
);
DEFINE_TLCL_EXPORT!(TlclGetSpaceInfo, crate::tlcl::commands::TlclGetSpaceInfo);
DEFINE_TLCL_EXPORT!(
  TlclGetPermissions,
  crate::tlcl::commands::TlclGetPermissions
);
DEFINE_TLCL_EXPORT!(
  TlclPhysicalPresenceCMDEnable,
  crate::tlcl::commands::TlclPhysicalPresenceCMDEnable
);
DEFINE_TLCL_EXPORT!(
  TlclAssertPhysicalPresence,
  crate::tlcl::commands::TlclAssertPhysicalPresence
);
DEFINE_TLCL_EXPORT!(TlclSetEnable, crate::tlcl::commands::TlclSetEnable);
DEFINE_TLCL_EXPORT!(TlclRead, crate::tlcl::commands::TlclRead);
DEFINE_TLCL_EXPORT!(
  TlclReadWithOffset,
  crate::tlcl::commands::TlclReadWithOffset
);
DEFINE_TLCL_EXPORT!(TlclNVReadPublic, crate::tlcl::commands::TlclNVReadPublic);
DEFINE_TLCL_EXPORT!(TlclStartup, crate::tlcl::commands::TlclStartup);
DEFINE_TLCL_EXPORT!(TlclSaveState, crate::tlcl::commands::TlclSaveState);
DEFINE_TLCL_EXPORT!(TlclResume, crate::tlcl::commands::TlclResume);
DEFINE_TLCL_EXPORT!(TlclWrite, crate::tlcl::commands::TlclWrite);
DEFINE_TLCL_EXPORT!(
  TlclWriteWithOffset,
  crate::tlcl::commands::TlclWriteWithOffset
);
