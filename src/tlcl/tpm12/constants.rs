// Copyright 2016 The ChromiumOS Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.
//
// Some TPM constants and type definitions for standalone compilation for use
// in the firmware

#![allow(non_camel_case_types)]
#![allow(non_snake_case, non_upper_case_globals)]

pub const TPM_MAX_COMMAND_SIZE: u32 = 4096;
pub const TPM_LARGE_ENOUGH_COMMAND_SIZE: u32 = 256; /* saves space in the firmware */
pub const TPM_PUBEK_SIZE: u32 = 256;
pub const TPM_PCR_DIGEST: u32 = 20;

pub const TPM_NV_INDEX0: u32 = 0x00000000u32;
pub const TPM_NV_INDEX_LOCK: u32 = 0xffffffffu32;
pub const TPM_NV_INDEX_TRIAL: u32 = 0x0000f004u32;

pub const TPM_NV_PER_READ_STCLEAR: u32 = 1u32 << 31;
pub const TPM_NV_PER_AUTHREAD: u32 = 1u32 << 18;
pub const TPM_NV_PER_OWNERREAD: u32 = 1u32 << 17;
pub const TPM_NV_PER_PPREAD: u32 = 1u32 << 16;
pub const TPM_NV_PER_GLOBALLOCK: u32 = 1u32 << 15;
pub const TPM_NV_PER_WRITE_STCLEAR: u32 = 1u32 << 14;
pub const TPM_NV_PER_WRITEDEFINE: u32 = 1u32 << 13;
pub const TPM_NV_PER_WRITEALL: u32 = 1u32 << 12;
pub const TPM_NV_PER_AUTHWRITE: u32 = 1u32 << 2;
pub const TPM_NV_PER_OWNERWRITE: u32 = 1u32 << 1;
pub const TPM_NV_PER_PPWRITE: u32 = 1u32 << 0;

pub const TPM_TAG_NV_ATTRIBUTES: u16 = 0x0017u16;
pub const TPM_TAG_NV_DATA_PUBLIC: u16 = 0x0018u16;
pub const TPM_TAG_KEY12: u16 = 0x0028u16;

pub const TPM_TAG_RQU_COMMAND: u16 = 0xc1u16;
pub const TPM_TAG_RQU_AUTH1_COMMAND: u16 = 0xc2u16;
pub const TPM_TAG_RQU_AUTH2_COMMAND: u16 = 0xc3u16;
pub const TPM_TAG_RSP_COMMAND: u16 = 0xc4u16;
pub const TPM_TAG_RSP_AUTH1_COMMAND: u16 = 0xc5u16;
pub const TPM_TAG_RSP_AUTH2_COMMAND: u16 = 0xc6u16;

pub type TSS_BOOL = u8;
pub type TPM_BOOL = u8;
pub type TPM_TAG = u16;
pub type TPM_STRUCTURE_TAG = u16;
pub type TPM_NV_INDEX = u32;
pub type TPM_NV_PER_ATTRIBUTES = u32;
pub type TPM_LOCALITY_SELECTION = u8;
pub type TPM_COMMAND_CODE = u32;
pub type TPM_PHYSICAL_PRESENCE = u16;
pub type TPM_STARTUP_TYPE = u16;
pub type TPM_CAPABILITY_AREA = u32;
pub type TPM_FAMILY_LABEL = u8;
pub type TPM_FAMILY_ID = u32;
pub type TPM_FAMILY_VERIFICATION = u32;
pub type TPM_FAMILY_FLAGS = u32;

pub const TPM_CAP_FLAG: u32 = 0x00000004u32;
pub const TPM_CAP_FLAG_PERMANENT: u32 = 0x00000108u32;
pub const TPM_CAP_FLAG_VOLATILE: u32 = 0x00000109u32;

pub const TPM_CAP_PROPERTY: u32 = 0x00000005u32;
pub const TPM_CAP_PROP_OWNER: u32 = 0x00000111u32;
pub const TPM_CAP_NV_INDEX: u32 = 0x00000011u32;
pub const TPM_CAP_GET_VERSION_VAL: u32 = 0x0000001au32;

pub const TPM_AUTH_ALWAYS: u8 = 0x01u8;
pub const TPM_KEY_USAGE_STORAGE: u16 = 0x0011u16;

pub const TPM_ALG_RSA: u16 = 0x0001u16;
pub const TPM_ES_RSAESOAEP_SHA1_MGF1: u16 = 0x0003u16;
pub const TPM_SS_NONE: u16 = 0x0001u16;

pub const TPM_PID_OWNER: u16 = 0x0005u16;
pub const TPM_ET_OWNER: u32 = 0x02u32;
pub const TPM_FAMILY_CREATE: u32 = 0x00000001u32;

pub const TPM_ST_CLEAR: u16 = 0x0001u16;
pub const TPM_ST_STATE: u16 = 0x0002u16;
pub const TPM_ST_DEACTIVATED: u16 = 0x0003u16;

pub const TPM_LOC_FOUR: u32 = 1u32 << 4;
pub const TPM_LOC_THREE: u32 = 1u32 << 3;
pub const TPM_LOC_TWO: u32 = 1u32 << 2;
pub const TPM_LOC_ONE: u32 = 1u32 << 1;
pub const TPM_LOC_ZERO: u32 = 1u32 << 0;
pub const TPM_ALL_LOCALITIES: u32 =
  TPM_LOC_ZERO | TPM_LOC_ONE | TPM_LOC_TWO | TPM_LOC_THREE | TPM_LOC_FOUR; /* 0x1f */

pub const TPM_PHYSICAL_PRESENCE_LOCK: u16 = 0x0004u16;
pub const TPM_PHYSICAL_PRESENCE_PRESENT: u16 = 0x0008u16;
pub const TPM_PHYSICAL_PRESENCE_NOTPRESENT: u16 = 0x0010u16;
pub const TPM_PHYSICAL_PRESENCE_CMD_ENABLE: u16 = 0x0020u16;
pub const TPM_PHYSICAL_PRESENCE_HW_ENABLE: u16 = 0x0040u16;
pub const TPM_PHYSICAL_PRESENCE_LIFETIME_LOCK: u16 = 0x0080u16;
pub const TPM_PHYSICAL_PRESENCE_CMD_DISABLE: u16 = 0x0100u16;
pub const TPM_PHYSICAL_PRESENCE_HW_DISABLE: u16 = 0x0200u16;

pub const TPM_SHA1_160_HASH_LEN: usize = 0x14;
pub const TPM_SHA1BASED_NONCE_LEN: usize = TPM_SHA1_160_HASH_LEN;
pub const TPM_AUTH_DATA_LEN: usize = 0x14;
pub const TPM_RSA_2048_LEN: usize = 0x100;

#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct TPM_DIGEST {
  pub digest: [u8; TPM_SHA1_160_HASH_LEN],
}

pub type TPM_COMPOSITE_HASH = TPM_DIGEST;

#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct TPM_PCR_SELECTION {
  pub sizeOfSelect: u16,
  pub pcrSelect: [u8; 3],
}

#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct TPM_NV_ATTRIBUTES {
  pub tag: TPM_STRUCTURE_TAG,
  pub attributes: TPM_NV_PER_ATTRIBUTES,
}

#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct TPM_PCR_INFO_SHORT {
  pub pcrSelection: TPM_PCR_SELECTION,
  pub localityAtRelease: TPM_LOCALITY_SELECTION,
  pub digestAtRelease: TPM_COMPOSITE_HASH,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct TPM_PERMANENT_FLAGS {
  pub tag: TPM_STRUCTURE_TAG,
  pub disable: TSS_BOOL,
  pub ownership: TSS_BOOL,
  pub deactivated: TSS_BOOL,
  pub readPubek: TSS_BOOL,
  pub disableOwnerClear: TSS_BOOL,
  pub allowMaintenance: TSS_BOOL,
  pub physicalPresenceLifetimeLock: TSS_BOOL,
  pub physicalPresenceHWEnable: TSS_BOOL,
  pub physicalPresenceCMDEnable: TSS_BOOL,
  pub CEKPUsed: TSS_BOOL,
  pub TPMpost: TSS_BOOL,
  pub TPMpostLock: TSS_BOOL,
  pub FIPS: TSS_BOOL,
  pub Operator: TSS_BOOL,
  pub enableRevokeEK: TSS_BOOL,
  pub nvLocked: TSS_BOOL,
  pub readSRKPub: TSS_BOOL,
  pub tpmEstablished: TSS_BOOL,
  pub maintenanceDone: TSS_BOOL,
  pub disableFullDALogicInfo: TSS_BOOL,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct TPM_STCLEAR_FLAGS {
  pub tag: TPM_STRUCTURE_TAG,
  pub deactivated: TSS_BOOL,
  pub disableForceClear: TSS_BOOL,
  pub physicalPresence: TSS_BOOL,
  pub physicalPresenceLock: TSS_BOOL,
  pub bGlobalLock: TSS_BOOL,
}

#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct TPM_NV_DATA_PUBLIC {
  pub tag: TPM_STRUCTURE_TAG,
  pub nvIndex: TPM_NV_INDEX,
  pub pcrInfoRead: TPM_PCR_INFO_SHORT,
  pub pcrInfoWrite: TPM_PCR_INFO_SHORT,
  pub permission: TPM_NV_ATTRIBUTES,
  pub bReadSTClear: TPM_BOOL,
  pub bWriteSTClear: TPM_BOOL,
  pub bWriteDefine: TPM_BOOL,
  pub dataSize: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct TPM_NONCE {
  pub nonce: [u8; TPM_SHA1BASED_NONCE_LEN],
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct TPM_FAMILY_TABLE_ENTRY {
  pub tag: TPM_STRUCTURE_TAG,
  pub familyLabel: TPM_FAMILY_LABEL,
  pub familyID: TPM_FAMILY_ID,
  pub verificationCount: TPM_FAMILY_VERIFICATION,
  pub flags: TPM_FAMILY_FLAGS,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct TPM_IFX_FIRMWAREPACKAGE {
  pub FwPackageIdentifier: u32,
  pub Version: u32,
  pub StaleVersion: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct TPM_IFX_FIELDUPGRADEINFO {
  pub wMaxDataSize: u16,
  pub sBootloaderFirmwarePackage: TPM_IFX_FIRMWAREPACKAGE,
  pub sFirmwarePackages: [TPM_IFX_FIRMWAREPACKAGE; 2],
  pub wSecurityModuleStatus: u16,
  pub sProcessFirmwarePackage: TPM_IFX_FIRMWAREPACKAGE,
  pub wFieldUpgradeCounter: u16,
}

#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct TPM_NV_AUTH_POLICY {
  pub pcr_info_read: TPM_PCR_INFO_SHORT,
  pub pcr_info_write: TPM_PCR_INFO_SHORT,
}

pub const TPM_IFX_FieldUpgradeInfoRequest2: u8 = 0x11u8;

/* Ordinals */
pub const TPM_ORD_ContinueSelfTest: u32 = 0x00000053u32;
pub const TPM_ORD_Delegate_Manage: u32 = 0x000000D2u32;
pub const TPM_ORD_Delegate_ReadTable: u32 = 0x000000DBu32;
pub const TPM_ORD_Extend: u32 = 0x00000014u32;
pub const TPM_ORD_FieldUpgrade: u32 = 0x000000AAu32;
pub const TPM_ORD_ForceClear: u32 = 0x0000005Du32;
pub const TPM_ORD_GetCapability: u32 = 0x00000065u32;
pub const TPM_ORD_GetRandom: u32 = 0x00000046u32;
pub const TPM_ORD_NV_DefineSpace: u32 = 0x000000CCu32;
pub const TPM_ORD_NV_ReadValue: u32 = 0x000000CFu32;
pub const TPM_ORD_NV_WriteValue: u32 = 0x000000CDu32;
pub const TPM_ORD_OIAP: u32 = 0x0000000Au32;
pub const TPM_ORD_OSAP: u32 = 0x0000000Bu32;
pub const TPM_ORD_PcrRead: u32 = 0x00000015u32;
pub const TPM_ORD_PhysicalEnable: u32 = 0x0000006Fu32;
pub const TPM_ORD_PhysicalDisable: u32 = 0x00000070u32;
pub const TSC_ORD_PhysicalPresence: u32 = 0x4000000Au32;
pub const TPM_ORD_PhysicalSetDeactivated: u32 = 0x00000072u32;
pub const TPM_ORD_ReadPubek: u32 = 0x0000007Cu32;
pub const TPM_ORD_SaveState: u32 = 0x00000098u32;
pub const TPM_ORD_SelfTestFull: u32 = 0x00000050u32;
pub const TPM_ORD_Startup: u32 = 0x00000099u32;
pub const TPM_ORD_TakeOwnership: u32 = 0x0000000Du32;

pub const TPM_SUCCESS: u32 = 0x00000000;
pub const TPM_BUFFER_SIZE: usize = 2048;
pub const TPM_CMD_HEADER_SIZE: usize = 10;
pub const TPM_RESP_HEADER_SIZE: usize = 10;

pub type TPM_COMMAND = u32;

#[repr(C)]
pub struct tpm1_nv_read_cmd {
  pub nvIndex: u32,
  pub offset: u32,
  pub size: u32,
}

#[repr(C)]
pub struct tpm1_nv_write_cmd {
  pub nvIndex: u32,
  pub offset: u32,
  pub size: u32,
  pub data: *const u8,
}

#[repr(C)]
pub struct tpm1_physical_presence_cmd {
  pub physical_presence: u16,
}

#[repr(C, packed)]
pub struct tpm_header {
  pub tpm_tag: u16,
  pub tpm_size: u32,
  pub tpm_code: u32,
}

#[repr(C)]
pub struct nv_read_response {
  pub data_size: u32,
  pub data: [u8; TPM_BUFFER_SIZE],
}

#[repr(C)]
pub struct tpm1_response {
  pub hdr: tpm_header,
  pub nvr: nv_read_response,
}
