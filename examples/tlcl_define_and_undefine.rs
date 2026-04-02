#[cfg(feature = "tpm1_2")]
use libcros::tlcl::{TlclAssertPhysicalPresence, TlclPhysicalPresenceCMDEnable};
use libcros::{
  LOG, Logger, kv_get, kv_set,
  libargs::ArgCheck,
  tlcl::{
    TlclDefineSpace, TlclUndefineSpace,
    permissions::{
      NV_PERM_AUTHREAD, NV_PERM_AUTHWRITE, NV_PERM_OWNERREAD, NV_PERM_OWNERWRITE, NV_PERM_PPREAD,
      NV_PERM_PPWRITE,
    },
  },
};

const NV_INDEX: u32 = 0x8000A;
const NV_SIZE: u32 = 0xA;

struct AuthType {
  name: &'static str,
  perm: u32,
}

const AUTH_TYPES: &[AuthType] = &[
  AuthType {
    name: "PPWRITE | PPREAD",
    perm: NV_PERM_PPWRITE | NV_PERM_PPREAD,
  },
  AuthType {
    name: "OWNERWRITE | OWNERREAD",
    perm: NV_PERM_OWNERWRITE | NV_PERM_OWNERREAD,
  },
  AuthType {
    name: "AUTHWRITE | AUTHREAD",
    perm: NV_PERM_AUTHWRITE | NV_PERM_AUTHREAD,
  },
  AuthType {
    name: "PPWRITE | AUTHREAD",
    perm: NV_PERM_PPWRITE | NV_PERM_AUTHREAD,
  },
  AuthType {
    name: "AUTHWRITE | PPREAD",
    perm: NV_PERM_AUTHWRITE | NV_PERM_PPREAD,
  },
];

fn main() {
  let mut args: ArgCheck = ArgCheck::new();
  let verbose: bool = args.fbool("--verbose", "", "Enable debug messages");
  let flags_tpm_path = args.fequals_str(
    "--tpm-path",
    "-t",
    "Specify a custom TPM device to use in /dev/tpmX format",
  );

  args.check_help();
  
  if flags_tpm_path.is_empty() {
    kv_set(libcros::keys::TPM_PATH, "/dev/tpm69");
  } else {
    kv_set(libcros::keys::TPM_PATH, &flags_tpm_path);
  }

  Logger::init(verbose, true);
  let tpm = kv_get(libcros::keys::TPM_PATH);

  LOG!("using {}", tpm);

  /* TPM 1.2 requires physical presence for DefineSpace with PPWRITE/PPREAD */
  #[cfg(feature = "tpm1_2")]
  {
    let rc = TlclPhysicalPresenceCMDEnable();
    LOG!("PhysicalPresenceCMDEnable rc: {:X}", rc);

    let rc = TlclAssertPhysicalPresence();
    LOG!("AssertPhysicalPresence rc: {:X}", rc);
  }

  for auth in AUTH_TYPES {
    LOG!("--| {} (perm=0x{:X}) |--", auth.name, auth.perm);

    let rc = TlclDefineSpace(NV_INDEX, auth.perm, NV_SIZE);
    LOG!("DefineSpace rc: {:X}", rc);

    if rc != 0 {
      LOG!("skipping undefine");
      continue;
    }

    let rc = TlclUndefineSpace(NV_INDEX);
    LOG!("UndefineSpace rc: {:X}", rc);
  }

  LOG!("done");
}
