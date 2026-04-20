use std::{fs::File, os::unix::net::UnixStream};

use crate::keyval::{KvValue, kv};

pub fn kv_set(key: &'static str, val: impl Into<KvValue>) {
  kv().lock().unwrap().insert(key, val.into());
}

impl From<String> for KvValue {
  fn from(v: String) -> Self {
    KvValue::String(v)
  }
}

impl From<&str> for KvValue {
  fn from(v: &str) -> Self {
    KvValue::String(v.to_string())
  }
}

impl From<i64> for KvValue {
  fn from(v: i64) -> Self {
    KvValue::Int(v)
  }
}

impl From<bool> for KvValue {
  fn from(v: bool) -> Self {
    KvValue::Bool(v)
  }
}

impl From<UnixStream> for KvValue {
  fn from(v: UnixStream) -> Self {
    KvValue::Socket(v)
  }
}

impl From<File> for KvValue {
  fn from(v: File) -> Self {
    KvValue::File(v)
  }
}

impl From<u8> for KvValue {
  fn from(v: u8) -> Self {
    KvValue::Int(v as i64)
  }
}

impl From<u16> for KvValue {
  fn from(v: u16) -> Self {
    KvValue::Int(v as i64)
  }
}

impl From<u32> for KvValue {
  fn from(v: u32) -> Self {
    KvValue::Int(v as i64)
  }
}
