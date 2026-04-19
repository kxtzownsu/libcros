use crate::keyval::kv;

pub fn kv_erase(key: &'static str) {
  kv().lock().unwrap().remove(key);
}