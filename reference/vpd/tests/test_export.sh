#!/bin/bash
#
# Copyright 2017 The ChromiumOS Authors
# Use of this source code is governed by a BSD-style license that can be
# found in the LICENSE file.
#
# Redistribution and use in source and binary forms, with or without
# modification, are permitted provided that the following conditions are
# met:
#
#    * Redistributions of source code must retain the above copyright
# notice, this list of conditions and the following disclaimer.
#    * Redistributions in binary form must reproduce the above
# copyright notice, this list of conditions and the following disclaimer
# in the documentation and/or other materials provided with the
# distribution.
#    * Neither the name of Google LLC nor the names of its
# contributors may be used to endorse or promote products derived from
# this software without specific prior written permission.
#
# THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
# "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
# LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
# A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
# OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
# SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
# LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
# DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
# THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
# (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
# OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
#
# Alternatively, this software may be distributed under the terms of the
# GNU General Public License ("GPL") version 2 as published by the Free
# Software Foundation.
set -e

. ./functions.sh

# shellcheck disable=SC2154 # exported by caller
BINARY="${OUT}/vpd"
TMP_DIR=$(mktemp -d)
BIOS_PACKS=( vpd_0x600.tbz gVpdInfo.tbz )
BIOS="${TMP_DIR}/empty.vpd"

test_image() {
  local pack="$1"

  echo "  testing '${pack}' ..."
  unpack_bios "${pack}" "${TMP_DIR}"

  local args=

  #
  # export a single vpd data
  RUN "${VPD_OK}" "${BINARY} -f ${BIOS} -O"
  RUN "${VPD_OK}" "${BINARY} -f ${BIOS} -s aaa=bbb"
  RUN "${VPD_OK}" "${BINARY} -f ${BIOS} -g aaa" "bbb"
  RUN "${VPD_FAIL}" "${BINARY} -f ${BIOS} -g xxx" ""

  #
  # export to shell script
  # Expect to get the same value after import back
  RUN "${VPD_OK}" "${BINARY} -f ${BIOS} -O"
  args="-f ${BIOS}"
  args="${args} -s 'a=aaa'"
  args="${args} -s $'b=b\\'b\\nb'"
  args="${args} -s 'c=cc'"
  args="${args} -s 'd=ddd'"
  args="${args} -s 'e='"
  args="${args} -s 'f=double\"quote'"
  args="${args} -s $'g=\\'\\'\\'\\''"
  args="${args} -l --sh"
  RUN "${VPD_OK}" "${BINARY} ${args} > ${TMP_DIR}/import.sh"
  RUN 0 "sh ${TMP_DIR}/import.sh"
  RUN "${VPD_OK}" "${BINARY} -f ${BIOS} -g a" "aaa"
  RUN "${VPD_OK}" "${BINARY} -f ${BIOS} -g b" $'b\'b\nb'
  RUN "${VPD_OK}" "${BINARY} -f ${BIOS} -g c" "cc"
  RUN "${VPD_OK}" "${BINARY} -f ${BIOS} -g d" "ddd"
  RUN "${VPD_OK}" "${BINARY} -f ${BIOS} -g e" ""
  RUN "${VPD_OK}" "${BINARY} -f ${BIOS} -g f" "double\"quote"
  RUN "${VPD_OK}" "${BINARY} -f ${BIOS} -g g" "''''"

  #
  # export to null-terminated format
  for arg_name in "--null-terminated" "-0"; do
    RUN "${VPD_OK}" "${BINARY} -f ${BIOS} -O"
    args="-f ${BIOS} -s a=aaa -s b=b -s c=ccc -l ${arg_name}"
    local expect_result=' 61 3d 61 61 61 00 62 3d 62 00 63 3d 63 63 63 00'
    RUN "${VPD_OK}" "${BINARY} ${args} | od -A n -t x1 -w32" "${expect_result}"
  done
}

main() {
  for pack in "${BIOS_PACKS[@]}"
  do
    test_image "${pack}"
  done
}

main
clean_up "${TMP_DIR}"

exit 0
