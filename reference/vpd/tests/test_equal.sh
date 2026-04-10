#!/bin/bash
#
# Copyright 2014 The ChromiumOS Authors
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

  #
  # Test only '=' in string
  # Expect = is saved successfully
  RUN "${VPD_OK}" "${BINARY} -f ${BIOS} -s KEY='='"
  RUN "${GREP_OK}" "${BINARY} -f ${BIOS} -l | grep '\"KEY\"=\"=\"'"

  #
  # '=' in the middle of string
  RUN "${VPD_OK}" "${BINARY} -f ${BIOS} -s KEY='123=456'"
  RUN "${GREP_OK}" "${BINARY} -f ${BIOS} -l | grep '\"123=456\"'"

  #
  # '=' in the begin of string
  RUN "${VPD_OK}" "${BINARY} -f ${BIOS} -s KEY='=5566'"
  RUN "${GREP_OK}" "${BINARY} -f ${BIOS} -l | grep '\"=5566\"'"

  #
  # '=' in the end of string
  RUN "${VPD_OK}" "${BINARY} -f ${BIOS} -s KEY='183='"
  RUN "${GREP_OK}" "${BINARY} -f ${BIOS} -l | grep '\"183=\"'"

  #
  # Multiple '=' in the string
  RUN "${VPD_OK}" "${BINARY} -f ${BIOS} -s KEY='==democracy=at=4am=='"
  RUN "${GREP_OK}" "${BINARY} -f ${BIOS} -l | grep '\"==democracy=at=4am==\"'"
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
