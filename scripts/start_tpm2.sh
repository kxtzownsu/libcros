#!/bin/bash

PORT=2322
TPM2_STATE_DIR=/usr/share/swtpm/tpm2
mkdir -p "$TPM2_STATE_DIR"

echo "starting tpm"
sudo swtpm chardev \
  --vtpm-proxy \
  --tpmstate dir="$TPM2_STATE_DIR" \
  --tpm2 \
  --log level=20 \
  --ctrl type=tcp,port="${PORT}"