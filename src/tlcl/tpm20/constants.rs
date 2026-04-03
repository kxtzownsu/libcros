/* Copyright 2016 The ChromiumOS Authors
 * Use of this source code is governed by a BSD-style license that can be
 * found in the LICENSE file.
 *
 * Some TPM constants and type definitions for standalone compilation for use
 * in the firmware
 */

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

pub const TPM_BUFFER_SIZE: usize = 2048;

/* Tpm2 command tags. */
pub const TPM_ST_NO_SESSIONS: u16 = 0x8001;
pub const TPM_ST_SESSIONS: u16 = 0x8002;

/* TPM2 command codes. */
pub const TPM2_EvictControl: TPM_CC = 0x00000120;
pub const TPM2_Hierarchy_Control: TPM_CC = 0x00000121;
pub const TPM2_NV_UndefineSpace: TPM_CC = 0x00000122;
pub const TPM2_Clear: TPM_CC = 0x00000126;
pub const TPM2_NV_DefineSpace: TPM_CC = 0x0000012A;
pub const TPM2_CreatePrimary: TPM_CC = 0x00000131;
pub const TPM2_NV_Write: TPM_CC = 0x00000137;
pub const TPM2_NV_WriteLock: TPM_CC = 0x00000138;
pub const TPM2_SelfTest: TPM_CC = 0x00000143;
pub const TPM2_Startup: TPM_CC = 0x00000144;
pub const TPM2_Shutdown: TPM_CC = 0x00000145;
pub const TPM2_NV_Read: TPM_CC = 0x0000014E;
pub const TPM2_NV_ReadLock: TPM_CC = 0x0000014F;
pub const TPM2_NV_ReadPublic: TPM_CC = 0x00000169;
pub const TPM2_ReadPublic: TPM_CC = 0x00000173;
pub const TPM2_GetCapability: TPM_CC = 0x0000017A;
pub const TPM2_GetRandom: TPM_CC = 0x0000017B;
pub const TPM2_PCR_Extend: TPM_CC = 0x00000182;

pub const TPM_HT_PCR: u32 = 0x00;
pub const TPM_HT_NV_INDEX: u32 = 0x01;

pub const HR_SHIFT: u32 = 24;
pub const HR_PCR: u32 = TPM_HT_PCR << HR_SHIFT;
pub const HR_NV_INDEX: u32 = TPM_HT_NV_INDEX << HR_SHIFT;
pub const TPM_RH_OWNER: u32 = 0x40000001;
pub const TPM_RH_NULL: u32 = 0x40000007;
pub const TPM_RH_ENDORSEMENT: u32 = 0x4000000B;
pub const TPM_RH_PLATFORM: u32 = 0x4000000C;
pub const TPM_RS_PW: u32 = 0x40000009;

/* TPM2 capabilities. */
pub const TPM_CAP_FIRST: TPM_CAP = 0x00000000;
pub const TPM_CAP_TPM_PROPERTIES: TPM_CAP = 0x00000006;

/* TPM properties */
pub const TPM_PT_NONE: TPM_PT = 0x00000000;
pub const PT_GROUP: TPM_PT = 0x00000100;
pub const PT_FIXED: TPM_PT = PT_GROUP;
pub const TPM_PT_MANUFACTURER: TPM_PT = PT_FIXED + 5;
pub const TPM_PT_VENDOR_STRING_1: TPM_PT = PT_FIXED + 6;
pub const TPM_PT_VENDOR_STRING_4: TPM_PT = PT_FIXED + 9;
pub const TPM_PT_FIRMWARE_VERSION_1: TPM_PT = PT_FIXED + 11;
pub const TPM_PT_FIRMWARE_VERSION_2: TPM_PT = PT_FIXED + 12;
pub const PT_VAR: TPM_PT = PT_GROUP * 2;
pub const TPM_PT_PERMANENT: TPM_PT = PT_VAR + 0;
pub const TPM_PT_STARTUP_CLEAR: TPM_PT = PT_VAR + 1;

/* TPM startup types. */
pub const TPM_SU_CLEAR: TPM_SU = 0x0000;
pub const TPM_SU_STATE: TPM_SU = 0x0001;

/* TPM algorithm IDs. */
pub const TPM_ALG_SHA1: TPM_ALG_ID = 0x0004;
pub const TPM_ALG_SHA256: TPM_ALG_ID = 0x000B;
pub const TPM_ALG_NULL: TPM_ALG_ID = 0x0010;

/* NV index attributes. */
pub const TPMA_NV_PPWRITE: TPMA_NV = 1 << 0;
pub const TPMA_NV_OWNERWRITE: TPMA_NV = 1 << 1;
pub const TPMA_NV_AUTHWRITE: TPMA_NV = 1 << 2;
pub const TPMA_NV_POLICYWRITE: TPMA_NV = 1 << 3;
pub const TPMA_NV_COUNTER: TPMA_NV = 1 << 4;
pub const TPMA_NV_BITS: TPMA_NV = 1 << 5;
pub const TPMA_NV_EXTEND: TPMA_NV = 1 << 6;
pub const TPMA_NV_POLICY_DELETE: TPMA_NV = 1 << 10;
pub const TPMA_NV_WRITELOCKED: TPMA_NV = 1 << 11;
pub const TPMA_NV_WRITEALL: TPMA_NV = 1 << 12;
pub const TPMA_NV_WRITEDEFINE: TPMA_NV = 1 << 13;
pub const TPMA_NV_WRITE_STCLEAR: TPMA_NV = 1 << 14;
pub const TPMA_NV_GLOBALLOCK: TPMA_NV = 1 << 15;
pub const TPMA_NV_PPREAD: TPMA_NV = 1 << 16;
pub const TPMA_NV_OWNERREAD: TPMA_NV = 1 << 17;
pub const TPMA_NV_AUTHREAD: TPMA_NV = 1 << 18;
pub const TPMA_NV_POLICYREAD: TPMA_NV = 1 << 19;
pub const TPMA_NV_NO_DA: TPMA_NV = 1 << 25;
pub const TPMA_NV_ORDERLY: TPMA_NV = 1 << 26;
pub const TPMA_NV_CLEAR_STCLEAR: TPMA_NV = 1 << 27;
pub const TPMA_NV_READLOCKED: TPMA_NV = 1 << 28;
pub const TPMA_NV_WRITTEN: TPMA_NV = 1 << 29;
pub const TPMA_NV_PLATFORMCREATE: TPMA_NV = 1 << 30;
pub const TPMA_NV_READ_STCLEAR: TPMA_NV = 1 << 31;

pub const TPMA_NV_MASK_READ: TPMA_NV =
  TPMA_NV_PPREAD | TPMA_NV_OWNERREAD | TPMA_NV_AUTHREAD | TPMA_NV_POLICYREAD;
pub const TPMA_NV_MASK_WRITE: TPMA_NV =
  TPMA_NV_PPWRITE | TPMA_NV_OWNERWRITE | TPMA_NV_AUTHWRITE | TPMA_NV_POLICYWRITE;

/* Starting indexes of NV index ranges, as defined in "Registry of reserved
 * TPM 2.0 handles and localities".
 */
pub const TPMI_RH_NV_INDEX_TPM_START: TPMI_RH_NV_INDEX = 0x01000000;
pub const TPMI_RH_NV_INDEX_PLATFORM_START: TPMI_RH_NV_INDEX = 0x01400000;
pub const TPMI_RH_NV_INDEX_OWNER_START: TPMI_RH_NV_INDEX = 0x01800000;
pub const TPMI_RH_NV_INDEX_TCG_OEM_START: TPMI_RH_NV_INDEX = 0x01C00000;
pub const TPMI_RH_NV_INDEX_TCG_WG_START: TPMI_RH_NV_INDEX = 0x01C40000;
pub const TPMI_RH_NV_INDEX_RESERVED_START: TPMI_RH_NV_INDEX = 0x01C90000;

pub const HASH_COUNT: usize = 1; /* Only SHA-256 is supported */

/* Table 206 - Defines for SHA256 Hash Values */
pub const SHA256_DIGEST_SIZE: usize = 32;

pub type TPMI_YES_NO = u8;
pub type TPM_CC = u32;
pub type TPM_HANDLE = u32;
pub type TPMI_DH_OBJECT = TPM_HANDLE;
pub type TPMI_DH_PCR = TPM_HANDLE;
pub type TPMI_DH_PERSISTENT = TPM_HANDLE;
pub type TPMI_RH_ENABLES = TPM_HANDLE;
pub type TPMI_RH_HIERARCHY = TPM_HANDLE;
pub type TPMI_RH_NV_INDEX = TPM_HANDLE;
pub type TPMI_RH_PROVISION = TPM_HANDLE;
pub type TPM_CAP = u32;
pub type TPM_PT = u32;
pub type TPM_SU = u16;
pub type TPM_ALG_ID = u16;
pub type TPMI_ALG_HASH = TPM_ALG_ID;
pub type TPMA_NV = u32;

#[repr(C)]
pub struct TPM2B {
  pub size: u16,
  pub buffer: *const u8,
}
pub type TPM2B_DIGEST = TPM2B;
pub type TPM2B_AUTH = TPM2B;
pub type TPM2B_NAME = TPM2B;

#[repr(C)]
pub struct TPMS_TAGGED_PROPERTY {
  pub property: TPM_PT,
  pub value: u32,
}

#[repr(C)]
pub struct TPML_TAGGED_TPM_PROPERTY {
  pub count: u32,
  pub tpm_property: [TPMS_TAGGED_PROPERTY; 1],
}

#[repr(C)]
pub union TPMU_HA {
  pub sha256: [u8; SHA256_DIGEST_SIZE],
}

#[repr(C)]
pub struct TPMT_HA {
  pub hashAlg: TPMI_ALG_HASH,
  pub digest: TPMU_HA,
}

#[repr(C)]
pub struct TPML_DIGEST_VALUES {
  pub count: u32,
  pub digests: [TPMT_HA; HASH_COUNT],
}

#[repr(C)]
pub union TPMU_CAPABILITIES {
  pub tpm_properties: core::mem::ManuallyDrop<TPML_TAGGED_TPM_PROPERTY>,
}

#[repr(C)]
pub struct TPMS_CAPABILITY_DATA {
  pub capability: TPM_CAP,
  pub data: TPMU_CAPABILITIES,
}

#[repr(C)]
pub struct TPMS_NV_PUBLIC {
  pub nvIndex: TPMI_RH_NV_INDEX,
  pub nameAlg: TPMI_ALG_HASH,
  pub attributes: TPMA_NV,
  pub authPolicy: TPM2B,
  pub dataSize: u16,
}

#[repr(C)]
pub struct tpm2_nv_define_space_cmd {
  pub auth: TPM2B,
  pub publicInfo: TPMS_NV_PUBLIC,
}

#[repr(C)]
pub struct tpm2_nv_undefine_space_cmd {
  pub nvIndex: TPMI_RH_NV_INDEX,
  pub use_platform_auth: u8,
}

#[repr(C)]
pub struct tpm2_nv_read_cmd {
  pub nvIndex: TPMI_RH_NV_INDEX,
  pub size: u16,
  pub offset: u16,
}

#[repr(C)]
pub struct tpm2_nv_write_cmd {
  pub nvIndex: TPMI_RH_NV_INDEX,
  pub data: TPM2B,
  pub offset: u16,
}

#[repr(C)]
pub struct tpm2_nv_read_lock_cmd {
  pub nvIndex: TPMI_RH_NV_INDEX,
}

#[repr(C)]
pub struct tpm2_nv_write_lock_cmd {
  pub nvIndex: TPMI_RH_NV_INDEX,
}

#[repr(C)]
pub struct tpm2_nv_read_public_cmd {
  pub nvIndex: TPMI_RH_NV_INDEX,
}

#[repr(C)]
pub struct tpm2_hierarchy_control_cmd {
  pub enable: TPMI_RH_ENABLES,
  pub state: TPMI_YES_NO,
}

#[repr(C)]
pub struct tpm2_get_capability_cmd {
  pub capability: TPM_CAP,
  pub property: u32,
  pub property_count: u32,
}

#[repr(C)]
pub struct tpm2_get_random_cmd {
  pub bytes_requested: u16,
}

#[repr(C)]
pub struct tpm2_self_test_cmd {
  pub full_test: TPMI_YES_NO,
}

#[repr(C)]
pub struct tpm2_startup_cmd {
  pub startup_type: TPM_SU,
}

#[repr(C)]
pub struct tpm2_shutdown_cmd {
  pub shutdown_type: TPM_SU,
}

#[repr(C)]
pub struct tpm2_pcr_extend_cmd {
  pub pcrHandle: TPMI_DH_PCR,
  pub digests: TPML_DIGEST_VALUES,
}

#[repr(C)]
pub struct tpm2_read_public_cmd {
  pub object_handle: TPMI_DH_OBJECT,
}

#[repr(C)]
pub struct tpm2_evict_control_cmd {
  pub auth: TPMI_RH_PROVISION,
  pub object_handle: TPMI_DH_OBJECT,
  pub persistent_handle: TPMI_DH_PERSISTENT,
}

#[repr(C)]
pub struct tpm2_create_primary_cmd {
  pub primary_handle: TPMI_RH_HIERARCHY,
  pub in_sensitive: TPM2B,
  pub in_public: TPM2B,
}

/* Common command/response header. */
#[repr(C, packed)]
pub struct tpm_header {
  pub tpm_tag: u16,
  pub tpm_size: u32,
  pub tpm_code: u32,
}

#[repr(C)]
pub struct nv_read_response {
  pub params_size: u32,
  pub buffer: TPM2B,
}

#[repr(C)]
pub struct read_public_response {
  pub buffer: TPM2B,
}

/* tpm2_session_attrs represents an 8-bit packed bitfield:
 *   bit 0:    continueSession
 *   bit 1:    auditExclusive
 *   bit 2:    auditReset
 *   bits 3-4: reserved3_4
 *   bit 5:    decrypt
 *   bit 6:    encrypt
 *   bit 7:    audit
 */
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct tpm2_session_attrs(pub u8);

impl tpm2_session_attrs {
  pub const CONTINUE_SESSION: u8 = 1 << 0;
  pub const AUDIT_EXCLUSIVE: u8 = 1 << 1;
  pub const AUDIT_RESET: u8 = 1 << 2;
  /* bits 3-4: reserved3_4 */
  pub const DECRYPT: u8 = 1 << 5;
  pub const ENCRYPT: u8 = 1 << 6;
  pub const AUDIT: u8 = 1 << 7;
}

#[repr(C)]
pub union tpm2_session_attrs_union {
  pub session_attr_bits: tpm2_session_attrs,
  pub session_attrs: u8,
}

#[repr(C)]
pub struct tpm2_session_header {
  pub session_handle: u32,
  pub nonce_size: u16,
  pub nonce: *mut u8,
  pub attrs: tpm2_session_attrs_union,
  pub auth_size: u16,
  pub auth: *mut u8,
}

#[repr(C, packed)]
pub struct get_capability_response {
  pub more_data: TPMI_YES_NO,
  pub capability_data: TPMS_CAPABILITY_DATA,
}

#[repr(C, packed)]
pub struct get_random_response {
  pub random_bytes: TPM2B_DIGEST,
}

#[repr(C, packed)]
pub struct nv_read_public_response {
  pub nvPublic: TPMS_NV_PUBLIC,
  pub nvName: TPM2B_NAME,
}

#[repr(C, packed)]
pub struct create_primary_response {
  pub object_handle: TPM_HANDLE,
}

#[repr(C)]
pub union tpm2_response_body {
  pub nvr: core::mem::ManuallyDrop<nv_read_response>,
  pub def_space: core::mem::ManuallyDrop<tpm2_session_header>,
  pub cap: core::mem::ManuallyDrop<get_capability_response>,
  pub random: core::mem::ManuallyDrop<get_random_response>,
  pub nv_read_public: core::mem::ManuallyDrop<nv_read_public_response>,
  pub read_pub: core::mem::ManuallyDrop<read_public_response>,
  pub create_primary: core::mem::ManuallyDrop<create_primary_response>,
}

#[repr(C)]
pub struct tpm2_response {
  pub hdr: tpm_header,
  pub body: tpm2_response_body,
}

/* TPM_PERMANENT_FLAGS represents a 32-bit packed bitfield:
 *   bit 0:      ownerAuthSet
 *   bit 1:      endorsementAuthSet
 *   bit 2:      lockoutAuthSet
 *   bits 3-7:   reserved3_7
 *   bit 8:      disableClear
 *   bit 9:      inLockout
 *   bit 10:     tpmGeneratedEPS
 *   bits 11-31: reserved11_31
 */
#[repr(transparent)]
pub struct TPM_PERMANENT_FLAGS(pub u32);

impl TPM_PERMANENT_FLAGS {
  pub const OWNER_AUTH_SET: u32 = 1 << 0;
  pub const ENDORSEMENT_AUTH_SET: u32 = 1 << 1;
  pub const LOCKOUT_AUTH_SET: u32 = 1 << 2;
  /* bits 3-7: reserved3_7 */
  pub const DISABLE_CLEAR: u32 = 1 << 8;
  pub const IN_LOCKOUT: u32 = 1 << 9;
  pub const TPM_GENERATED_EPS: u32 = 1 << 10;
  /* bits 11-31: reserved11_31 */
}

/* TPM_STCLEAR_FLAGS represents a 32-bit packed bitfield:
 *   bit 0:     phEnable
 *   bit 1:     shEnable
 *   bit 2:     ehEnable
 *   bit 3:     phEnableNV
 *   bits 4-30: reserved4_30
 *   bit 31:    orderly
 */
#[repr(transparent)]
pub struct TPM_STCLEAR_FLAGS(pub u32);

impl TPM_STCLEAR_FLAGS {
  pub const PH_ENABLE: u32 = 1 << 0;
  pub const SH_ENABLE: u32 = 1 << 1;
  pub const EH_ENABLE: u32 = 1 << 2;
  pub const PH_ENABLE_NV: u32 = 1 << 3;
  /* bits 4-30: reserved4_30 */
  pub const ORDERLY: u32 = 1 << 31;
}

#[repr(C)]
pub struct TPM_IFX_FIELDUPGRADEINFO;

/* TODO(apronin): For TPM2 certain properties must be received using
 * TPM2_GetCapability instead of being hardcoded as they are now:
 * TPM_MAX_COMMAND_SIZE -> use TPM_PT_MAX_COMMAND_SIZE for TPM2.
 * TPM_PCR_DIGEST -> use TPM_PT_MAX_DIGEST for TPM2.
 */
pub const TPM_MAX_COMMAND_SIZE: usize = 4096;
pub const TPM_PCR_DIGEST: usize = 32;
