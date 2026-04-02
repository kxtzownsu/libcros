#![allow(non_snake_case)]

/*
These aren't implemented in the original Tlcl on TPM 2.0.

Is returning 0x00 correct here?
*/

pub fn TlclPhysicalPresenceCMDEnable() -> u32 {
  0x00
}

pub fn TlclAssertPhysicalPresence() -> u32 {
  0x00
}

pub fn TlclSetEnable() -> u32 {
  0x00
}
