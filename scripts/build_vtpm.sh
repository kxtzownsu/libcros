#!/bin/bash

set -euo pipefail

KERNEL_TAG=$(uname -r | sed 's/\([0-9]*\.[0-9]*\).*/v\1/')
BASE_URL="https://raw.githubusercontent.com/torvalds/linux/${KERNEL_TAG}/drivers/char/tpm"
BUILD_DIR=$(mktemp -d)

file_list=(
  "tpm_vtpm_proxy.c"
  "tpm.h"
)

for file in "${file_list[@]}"; do
  wget "${BASE_URL}/${file}" -O "${BUILD_DIR}/${file}"
done

cat <<EOF > "${BUILD_DIR}/Makefile"
obj-m += tpm_vtpm_proxy.o

all:
	make -C /lib/modules/\$(shell uname -r)/build M=${BUILD_DIR} modules

clean:
	make -C /lib/modules/\$(shell uname -r)/build M=${BUILD_DIR} clean
EOF

make -C "${BUILD_DIR}"
if [ -f "${BUILD_DIR}/tpm_vtpm_proxy.ko" ]; then
  insmod "${BUILD_DIR}/tpm_vtpm_proxy.ko"
else
  echo "failed to build tpm_vtpm_proxy"
  exit 1
fi