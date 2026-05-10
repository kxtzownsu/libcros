#!/bin/bash

set -euo pipefail

if [ $# -lt 1 ]; then
  echo "usage: $0 <output.img>" >&2
  exit 1
fi

OUT="$1"

# fat tools (arch uses mkfs.vfat)
if command -v mkfs.fat >/dev/null 2>&1; then
  FAT_CMD=mkfs.fat
elif command -v mkfs.vfat >/dev/null 2>&1; then
  FAT_CMD=mkfs.vfat
else
  echo "mkfs.fat/mkfs.vfat not found (install dosfstools)" >&2
  exit 1
fi


# sizes are in MiB
P1_START=1
P1_SIZE=8

P2_START=$((P1_START + P1_SIZE))
P2_SIZE=16

P3_START=$((P2_START + P2_SIZE))
P3_SIZE=64

END=$((P3_START + P3_SIZE + 2)) # 2MiB padding
SIZE_MB="$END"

echo "creating image: $OUT (${SIZE_MB}MB)"
dd if=/dev/zero of="$OUT" bs=1M count="$SIZE_MB"

LOOP=$(losetup --show -fP "$OUT")
echo "loop device: $LOOP"

cleanup() {
  echo "cleaning up"
  losetup -d "$LOOP"
}
trap cleanup EXIT

echo "creating gpt + partitions"

parted -s "$LOOP" mklabel gpt

parted -s "$LOOP" mkpart primary ext4 ${P1_START}MiB $((P1_START + P1_SIZE))MiB
parted -s "$LOOP" mkpart primary fat16 ${P2_START}MiB $((P2_START + P2_SIZE))MiB
parted -s "$LOOP" mkpart primary fat32 ${P3_START}MiB $((P3_START + P3_SIZE))MiB

partprobe "$LOOP"
sleep 1

P1="${LOOP}p1"
P2="${LOOP}p2"
P3="${LOOP}p3"

echo "formatting filesystems"

mkfs.ext4 -F "$P1"
$FAT_CMD -F 16 "$P2"
$FAT_CMD -F 32 "$P3"

echo "done: $OUT"