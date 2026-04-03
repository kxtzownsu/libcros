/* Copyright 2013 The ChromiumOS Authors
 * Use of this source code is governed by a BSD-style license that can be
 * found in the LICENSE file.
 */

pub const TPM_SUCCESS: u32 = 0x00000000;

pub const TPM_E_ALREADY_INITIALIZED: u32 = 0x00005000; /* vboot local */
pub const TPM_E_INTERNAL_INCONSISTENCY: u32 = 0x00005001; /* vboot local */
pub const TPM_E_MUST_REBOOT: u32 = 0x00005002; /* vboot local */
pub const TPM_E_CORRUPTED_STATE: u32 = 0x00005003; /* vboot local */
pub const TPM_E_COMMUNICATION_ERROR: u32 = 0x00005004; /* vboot local */
pub const TPM_E_RESPONSE_TOO_LARGE: u32 = 0x00005005; /* vboot local */
pub const TPM_E_NO_DEVICE: u32 = 0x00005006; /* vboot local */
pub const TPM_E_INPUT_TOO_SMALL: u32 = 0x00005007; /* vboot local */
pub const TPM_E_WRITE_FAILURE: u32 = 0x00005008; /* vboot local */
pub const TPM_E_READ_EMPTY: u32 = 0x00005009; /* vboot local */
pub const TPM_E_READ_FAILURE: u32 = 0x0000500a; /* vboot local */
pub const TPM_E_STRUCT_SIZE: u32 = 0x0000500b; /* vboot local */
pub const TPM_E_STRUCT_VERSION: u32 = 0x0000500c; /* vboot local */
pub const TPM_E_INTERNAL_ERROR: u32 = 0x0000500d; /* vboot local */
pub const TPM_E_INVALID_RESPONSE: u32 = 0x0000500e; /* vboot local */
pub const TPM_E_BUFFER_SIZE: u32 = 0x0000500f; /* vboot local */
pub const TPM_E_NO_SUCH_COMMAND: u32 = 0x00005010; /* vboot local */

/*
 * AP firmware relies on Tlcl returning these exact TPM1.2 error codes
 * regardless of the TPM spec version in certain sitautions. So, TPM2.0 should
 * map to these errors when necessary. All TPM2.0-spec-defined errors have
 * either 0x100 or 0x80 bit set, so there is no confusion with actual error
 * codes returned from a TPM2.0 chip.
 */
pub const TPM_E_AUTHFAIL: u32 = 0x00000001;
pub const TPM_E_BADINDEX: u32 = 0x00000002;
pub const TPM_E_BAD_ORDINAL: u32 = 0x0000000a;
pub const TPM_E_OWNER_SET: u32 = 0x00000014;
pub const TPM_E_BADTAG: u32 = 0x0000001e;
pub const TPM_E_IOERROR: u32 = 0x0000001f;
pub const TPM_E_INVALID_POSTINIT: u32 = 0x00000026;
pub const TPM_E_BAD_PRESENCE: u32 = 0x0000002d;
pub const TPM_E_AREA_LOCKED: u32 = 0x0000003c;
pub const TPM_E_MAXNVWRITES: u32 = 0x00000048;

pub const TPM_E_NON_FATAL: u32 = 0x800;
pub const TPM_E_NEEDS_SELFTEST: u32 = TPM_E_NON_FATAL + 1;
pub const TPM_E_DOING_SELFTEST: u32 = TPM_E_NON_FATAL + 2;
