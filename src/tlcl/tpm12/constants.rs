#![allow(non_upper_case_globals)]

/*
Sources:
https://chromium.googlesource.com/chromiumos/platform/vboot_reference/+/e388d1f93c9573a79a04b633c3a0569ddbce6c94/firmware/include/tpm1_tss_constants.h
https://chromium.googlesource.com/chromiumos/platform/vboot_reference/+/e388d1f93c9573a79a04b633c3a0569ddbce6c94/firmware/include/tss_constants.h
*/

/* command tags */
pub const TPM_TAG_RQU_COMMAND: u16 = 0x00C1;
pub const TPM_TAG_RSP_COMMAND: u16 = 0x00C4;

/* command ordinals */
pub const TPM_ORD_NV_ReadValue: u32 = 0x000000CF;
pub const TPM_ORD_NV_WriteValue: u32 = 0x000000CD;
pub const TPM_ORD_ForceClear: u32 = 0x0000005D;

/* response codes */
pub const TPM_SUCCESS: u32 = 0x00000000;
pub const TPM_E_BADINDEX: u32 = 0x00000002;
pub const TPM_E_RETRY: u32 = 0x00000800;

/* not from spec - controls busy handling */
pub const TPM_MAX_RETRIES: u32 = 5;
pub const TPM_RETRY_DELAY_MS: u64 = 100;

/* sizes */
pub const TPM_CMD_HEADER_SIZE: usize = 10;
pub const TPM_WRITE_INFO_SIZE: usize = 12; /* nvIndex(4) + nvOffset(4) + dataSize(4) */
pub const TPM_RESP_HEADER_SIZE: usize = 10;

/* permissions */
pub const TPM_NV_PER_PPWRITE: u32 = 1 << 0;
pub const TPM_NV_PER_OWNERWRITE: u32 = 1 << 1;
pub const TPM_NV_PER_AUTHWRITE: u32 = 1 << 2;
pub const TPM_NV_PER_POLICYWRITE: u32 = 1 << 3;
pub const TPM_NV_PER_GLOBALLOCK: u32 = 1 << 15;
pub const TPM_NV_PER_PPREAD: u32 = 1 << 16;
pub const TPM_NV_PER_OWNERREAD: u32 = 1 << 17;
pub const TPM_NV_PER_AUTHREAD: u32 = 1 << 18;
pub const TPM_NV_PER_READ_STCLEAR: u32 = 1 << 31;
pub const TPM_NV_PER_WRITE_STCLEAR: u32 = 1 << 14;
pub const TPM_NV_PER_WRITEDEFINE: u32 = 1 << 13;
pub const TPM_NV_PER_WRITEALL: u32 = 1 << 12;
