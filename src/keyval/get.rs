use crate::keyval::{kv, KvValue};

pub fn kv_get(ty: &str, key: &'static str) -> Option<KvValue> {
  let map = kv().lock().unwrap();
  let val = map.get(key)?;

  match (ty, val) {
    ("string", KvValue::String(_)) => Some(clone_string(val)),
    ("int", KvValue::Int(_)) => Some(clone_int(val)),
    ("bool", KvValue::Bool(_)) => Some(clone_bool(val)),
    ("socket", KvValue::Socket(_)) => None,
    ("file", KvValue::File(f)) => f.try_clone().ok().map(KvValue::File),
    _ => None,
  }
}

fn clone_string(v: &KvValue) -> KvValue {
  if let KvValue::String(s) = v {
    KvValue::String(s.clone())
  } else {
    unreachable!()
  }
}

fn clone_int(v: &KvValue) -> KvValue {
  if let KvValue::Int(i) = v {
    KvValue::Int(*i)
  } else {
    unreachable!()
  }
}

fn clone_bool(v: &KvValue) -> KvValue {
  if let KvValue::Bool(b) = v {
    KvValue::Bool(*b)
  } else {
    unreachable!()
  }
}