#!/bin/bash
#
# Copyright 2013 The ChromiumOS Authors
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
  # Add 3 strings one time.
  # Expect SUCCESS
  RUN "${VPD_OK}" "${BINARY} -f ${BIOS} -s K0=D0 -s K1=D1 -s K2=D2"

  #
  # Add one more string and replace one.
  # Expect SUCCESS
  RUN "${VPD_OK}" "${BINARY} -f ${BIOS} -s K3=D3 -s K1=d1"
  # Make sure K3 has been added
  RUN "${GREP_OK}" "${BINARY} -f ${BIOS} -l | grep K3"
  # Make sure K1 has been replaced
  RUN "${GREP_OK}" "${BINARY} -f ${BIOS} -l | grep K1 | grep d1"

  #
  # Test -d K99.
  # expect error because non-existed key
  RUN "${VPD_ERR_PARAM}" "${BINARY} -f ${BIOS} -d K99"

  #
  # Test -d K99 and -d K2.
  # expect error because not all key is found.
  RUN "${VPD_ERR_PARAM}" "${BINARY} -f ${BIOS} -d K99 -d K2"
  # Make sure K2 is still existing.
  RUN "${GREP_OK}" "${BINARY} -f ${BIOS} -l | grep K2"

  #
  # Delete single.
  # expect SUCCESS
  RUN "${VPD_OK}" "${BINARY} -f ${BIOS} -d K2"

  #
  # Delete remaining string (K0, K1, K3)
  # expect SUCCESS and nothing left.
  RUN "${VPD_OK}" "${BINARY} -f ${BIOS} -d K1 -d K0 -d K3"
  RUN "${GREP_FAIL}" "${BINARY} -f ${BIOS} -l | grep -e '.*'"

  #
  # -s -s, then -d -d
  # expect SUCCESS and nothing left.
  RUN "${VPD_OK}" "${BINARY} -f ${BIOS} -s K98 -s K99 -d K98 -d K99"
  RUN "${GREP_FAIL}" "${BINARY} -f ${BIOS} -l | grep -e '.*'"
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
