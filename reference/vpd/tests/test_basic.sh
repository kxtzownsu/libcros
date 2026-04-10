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
  # List only
  # Expect SUCCESS and the mtime is NOT changed
  org_mtime=$(mtime "${BIOS}")
  org_chksum=$(chksum "${BIOS}")
  sleep 2  # ensure the system has changed.
  RUN "${VPD_OK}" "${BINARY} -f ${BIOS} -l"
  new_mtime=$(mtime "${BIOS}")
  new_chksum=$(chksum "${BIOS}")
  EXPECT_EQ "${org_mtime}" "${new_mtime}" \
            "The file mtime has been modified unexpectedly."
  EXPECT_EQ "${org_chksum}" "${new_chksum}" \
            "The file content has been modified unexpectedly."

  #
  # -O to reset all
  # expect SUCCESS and nothing exists
  RUN "${VPD_OK}" "${BINARY} -f ${BIOS} -O"
  RUN "${GREP_FAIL}" "${BINARY} -f ${BIOS} -l | grep -e '.*'"

  #
  # Add one string
  # Expect SUCCESS
  RUN "${VPD_OK}" "${BINARY} -f ${BIOS} -s ABC=DEF"
  # Make sure ABC has been added
  RUN "${GREP_OK}" "${BINARY} -f ${BIOS} -l | grep ABC | grep DEF"
  # Check format, expect the key string is in front of value string
  RUN "${GREP_OK}" "${BINARY} -f ${BIOS} -l | grep 'ABC.*DEF'"

  #
  # Test replacement
  RUN "${VPD_OK}" "${BINARY} -f ${BIOS} -s ABC=123"
  # Make sure ABC has been replaced
  RUN "${GREP_OK}" "${BINARY} -f ${BIOS} -l | grep ABC | grep 123"

  #
  # Test -d NONE
  # expect error because non-existed key
  RUN "${VPD_ERR_PARAM}" "${BINARY} -f ${BIOS} -d NONE"

  #
  # Test adding strings with double quote (")
  # Serial number is a special case.
  RUN "${VPD_ERR_PARAM}" "${BINARY} -f ${BIOS} -s serial_number=quote\\\"quote"
  # For other keys, double quote can be used.
  RUN "${VPD_OK}" "${BINARY} -f ${BIOS} -s XYZ=quote\\\"quote"
  # Make sure the value is as expected
  RUN "${GREP_OK}" "${BINARY} -f ${BIOS} -l | grep XYZ | grep quote\\\"quote"
  # Clean up.
  RUN "${VPD_OK}" "${BINARY} -f ${BIOS} -d XYZ"

  #
  # Delete single.
  # expect SUCCESS and nothing left.
  RUN "${VPD_OK}" "${BINARY} -f ${BIOS} -d ABC"
  RUN "${GREP_FAIL}" "${BINARY} -f ${BIOS} -l | grep -e '.*'"

  #
  # Test -O and -s at the same command
  # expect SUCCESS and new key has been added
  RUN "${VPD_OK}" "${BINARY} -f ${BIOS} -O -s 5566=183"
  RUN "${GREP_OK}" "${BINARY} -f ${BIOS} -l | grep 5566 | grep 183"

  #
  # -O to reset all
  # expect SUCCESS and nothing exists
  RUN "${VPD_OK}" "${BINARY} -f ${BIOS} -O"
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
