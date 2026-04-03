#!/bin/bash

PORT=2321
if [ -n "$GITHUB_WORKFLOW" ]; then
  TPM1_STATE_DIR=/tmp/swtpm/tpm1
else
  TPM1_STATE_DIR=/usr/share/swtpm/tpm1
fi
mkdir -p "$TPM1_STATE_DIR"

if command -v swtpm_setup >/dev/null 2>&1; then
  SWTPM_SETUP_CMD=swtpm_setup
elif command -v swtpm-setup >/dev/null 2>&1; then
  SWTPM_SETUP_CMD=swtpm-setup
else
  echo "swtpm setup tool not found (expected swtpm_setup or swtpm-setup)" >&2
  exit 1
fi

# do we need to init tpm? (only needed on tpm 1.2)
if [ ! -f "$TPM1_STATE_DIR/tpm-00.permall" ]; then
  echo "initializing tpm"
  sudo "$SWTPM_SETUP_CMD" \
    --tpmstate "$TPM1_STATE_DIR" \
    --createek \
    --create-platform-cert
  if [ $? -ne 0 ]; then
    echo "swtpm_setup failed. Aborting." >&2
    exit 1
  fi
  echo "setup done"
else
  echo "tpm already initialized"
fi

echo "starting tpm"
sudo swtpm chardev \
  --vtpm-proxy \
  --tpmstate dir="$TPM1_STATE_DIR" \
  --log level=20 \
  --locality allow-set-locality \
  --ctrl type=tcp,port="${PORT}"
