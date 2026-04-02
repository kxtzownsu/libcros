use libcros::{
  LOG, Logger, kv_set,
  tlcl::{
    TlclAssertPhysicalPresence, TlclForceClear, TlclPhysicalPresenceCMDEnable, TlclSetEnable,
  },
};

fn main() {
  Logger::init(true, true);
  let tpm = "/dev/tpm1";
  kv_set(libcros::keys::TPM_PATH, tpm);
  LOG!("clearing {}", tpm);

  let rc = TlclPhysicalPresenceCMDEnable();
  LOG!("PhysicalPresenceCMDEnable rc: {:X}", rc);

  let rc = TlclAssertPhysicalPresence();
  LOG!("AssertPhysicalPresence rc: {:X}", rc);

  let rc = TlclForceClear();
  LOG!("ForceClear rc: {:X}", rc);

  let rc = TlclSetEnable();
  LOG!("SetEnable rc: {:X}", rc);
}
