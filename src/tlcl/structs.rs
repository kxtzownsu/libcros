#![allow(non_snake_case, dead_code)]
#[cfg(feature = "tpm2_0")]
use std::fmt;
use std::mem::ManuallyDrop;

use crate::structs::{TPM2B, Tpm2SessionHeader, Tpm2TpmHeader};

#[derive(Debug)]
pub struct NvReadResponse {
  pub params_size: u32,
  pub buffer: TPM2B,
}

#[derive(Debug)]
pub struct TpmsTaggedProperty {
  pub property: u32,
  pub value: u32,
}

#[derive(Debug)]
pub struct TpmlTaggedTpmProperty {
  pub count: u32,
  pub tpm_property: [TpmsTaggedProperty; 1],
}

pub union TpmuCapabilities {
  pub tpm_properties: ManuallyDrop<TpmlTaggedTpmProperty>,
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
pub struct TpmsCapabilityData {
  pub capability: u32,
  pub data: TpmuCapabilities,
}

#[derive(Debug)]
pub struct GetCapabilityResponse {
  pub more_data: u8,
  pub capability_data: TpmsCapabilityData,
}

#[derive(Debug)]
pub struct GetRandomResponse {
  pub random_bytes: TPM2B,
}

#[derive(Debug)]
pub struct TpmsNvPublic {
  pub nvIndex: u32,
  pub nameAlg: u16,
  pub attributes: u32,
  pub authPolicy: TPM2B,
  pub dataSize: u16,
}

#[derive(Debug)]
pub struct NvReadPublicResponse {
  pub nvPublic: TpmsNvPublic,
  pub nvName: TPM2B,
}

#[derive(Debug)]
pub struct ReadPublicResponse {
  pub buffer: TPM2B,
}

#[derive(Debug)]
pub struct CreatePrimaryResponse {
  pub object_handle: u32,
}

pub union Tpm2ResponseData {
  pub nvr: ManuallyDrop<NvReadResponse>,
  pub def_space: ManuallyDrop<Tpm2SessionHeader>,
  pub cap: ManuallyDrop<GetCapabilityResponse>,
  pub random: ManuallyDrop<GetRandomResponse>,
  pub nv_read_public: ManuallyDrop<NvReadPublicResponse>,
  pub read_pub: ManuallyDrop<ReadPublicResponse>,
  pub create_primary: ManuallyDrop<CreatePrimaryResponse>,
}

impl fmt::Debug for Tpm2ResponseData {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    unsafe {
      f.debug_struct("Tpm2ResponseData")
        .field("nvr", &self.nvr)
        .field("def_space", &self.def_space)
        .field("cap", &self.cap)
        .field("random", &self.random)
        .field("nv_read_public", &self.nv_read_public)
        .field("read_pub", &self.read_pub)
        .field("create_primary", &self.create_primary)
        .finish()
    }
  }
}

#[derive(Debug)]
pub struct Tpm2Response {
  pub hdr: Tpm2TpmHeader,
  pub data: Tpm2ResponseData,
}
