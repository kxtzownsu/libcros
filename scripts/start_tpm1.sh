#!/bin/bash

PORT=2321
TPM1_STATE_DIR=/usr/share/swtpm/tpm1
mkdir -p "$TPM1_STATE_DIR"

# do we need to init tpm? (only needed on tpm 1.2)
if [ ! -f "$TPM1_STATE_DIR/tpm-00.permall" ]; then
  echo "initializing tpm"
  sudo swtpm_setup \
    --tpmstate "$TPM1_STATE_DIR" \
    --createek \
    --create-platform-cert \
    --lock-nvram
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
  --ctrl type=tcp,port="${PORT}"