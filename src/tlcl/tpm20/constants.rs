#![allow(non_upper_case_globals)]

/*
Sources:
https://chromium.googlesource.com/chromiumos/platform/vboot_reference/+/e388d1f93c9573a79a04b633c3a0569ddbce6c94/firmware/include/tpm2_tss_constants.h
https://chromium.googlesource.com/chromiumos/third_party/tpm2/+/e2282558e2e1ffc992078f457539108eff1af246/tpm_types.h
*/

/* session tag constants */
pub const TPM_ST_NO_SESSIONS: u16 = 0x8001;
pub const TPM_ST_SESSIONS: u16 = 0x8002;

/* hierarchy/session handles */
pub const TPM_RS_PW: u32 = 0x40000009;
pub const TPM_RH_PLATFORM: u32 = 0x4000000C;
pub const TPM_RH_OWNER: u32 = 0x40000001;
pub const TPMI_RH_NV_INDEX_OWNER_START: u32 = 0x01800000;
pub const HR_NV_INDEX: u32 = 0x01000000;

/* permissions */
pub const TPMA_NV_PPWRITE: u32 = 1 << 0;
pub const TPMA_NV_OWNERWRITE: u32 = 1 << 1;
pub const TPMA_NV_AUTHWRITE: u32 = 1 << 2;
pub const TPMA_NV_POLICYWRITE: u32 = 1 << 3;
pub const TPMA_NV_WRITEDEFINE: u32 = 1 << 13;
pub const TPMA_NV_WRITE_STCLEAR: u32 = 1 << 14;
pub const TPMA_NV_PPREAD: u32 = 1 << 16;
pub const TPMA_NV_OWNERREAD: u32 = 1 << 17;
pub const TPMA_NV_AUTHREAD: u32 = 1 << 18;
pub const TPMA_NV_POLICYREAD: u32 = 1 << 19;
pub const TPMA_NV_READ_STCLEAR: u32 = 1 << 31;
pub const TPMA_NV_PLATFORMCREATE: u32 = 1 << 30;

pub const TPMA_NV_MASK_WRITE: u32 =
  TPMA_NV_PPWRITE | TPMA_NV_OWNERWRITE | TPMA_NV_AUTHWRITE | TPMA_NV_POLICYWRITE;
pub const TPMA_NV_MASK_READ: u32 =
  TPMA_NV_PPREAD | TPMA_NV_OWNERREAD | TPMA_NV_AUTHREAD | TPMA_NV_POLICYREAD;

/* command codes */
pub const TPM2_Clear: u32 = 0x126;
pub const TPM2_NV_Read: u32 = 0x14E;
pub const TPM2_NV_ReadPublic: u32 = 0x169;
pub const TPM2_NV_Write: u32 = 0x137;
pub const TPM2_NV_DefineSpace: u32 = 0x12A;
pub const TPM2_NV_UndefineSpace: u32 = 0x122;

/* response codes */
pub const TPM_RC_SUCCESS: u32 = 0x000;
pub const TPM_RC_HANDLE: u32 = 0x18B; /* RC_VER1(0x100) + RC_FMT1(0x80) + 0x0B */
pub const TPM_RC_RETRY: u32 = 0x922; /* RC_WARN(0x900) + 0x22 */

pub const TPM_CMD_HEADER_SIZE: usize = 10;

pub const TPM_ALG_SHA256: u16 = 0x000B;

/* not from ChromiumOS, controls busy handling */
pub const TPM_MAX_RETRIES: u32 = 5;
pub const TPM_RETRY_DELAY_MS: u64 = 100;
