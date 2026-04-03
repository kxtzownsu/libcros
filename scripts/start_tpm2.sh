#!/bin/bash

PORT=2322
if [ -n "$GITHUB_WORKFLOW" ]; then
  TPM2_STATE_DIR=/tmp/swtpm/tpm2
else
  TPM2_STATE_DIR=/usr/share/swtpm/tpm2
fi
mkdir -p "$TPM2_STATE_DIR"

echo "starting tpm"
sudo swtpm chardev \
  --vtpm-proxy \
  --tpmstate dir="$TPM2_STATE_DIR" \
  --tpm2 \
  --log level=20 \
  --ctrl type=tcp,port="${PORT}"