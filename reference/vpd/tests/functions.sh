#!/bin/sh
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
#
# shellcheck disable=SC2034 # This is a library that exports a bunch of
#                           # seemingly-unused stuff.
set -e

# Error/exit codes defined in lib_vpd.h.
VPD_OK=0
VPD_FAIL=3
VPD_ERR_SYSTEM=4
VPD_ERR_ROM_READ=5
VPD_ERR_ROM_WRITE=6
VPD_ERR_SYNTAX=7
VPD_ERR_PARAM=8
VPD_ERR_NOT_FOUND=9
VPD_ERR_OVERFLOW=10
VPD_ERR_INVALID=11
VPD_ERR_DECODE=12

GREP_OK=0
GREP_FAIL=1

clean_up() {
  rm -rf "$1"
}

# $1: the tbz file
# $2: target directory
unpack_bios() {
  tar -axf "$1" -C "$2"
}

#
# die $3 if $1 != $2
#
EXPECT_EQ() {
  if [ "$1" != "$2" ]; then
    echo "[FAIL] EXPECT_EQ(expect:$2 exact:$1): $3"
    exit 1
  fi
}

#
# run command in $2 and expect exit code $1
# if $3 is given, also checks whether the command output is equal to $3
#
RUN() {
  local rc
  local output
  output=$(eval "$2") && rc=$? || rc=$?

  EXPECT_EQ "${rc}" "$1" "$2"
  if [ $# -gt 2 ]; then
    EXPECT_EQ "${output}" "$3" "$2"
  fi
}

#
# Get a finger print of a file (to ensure the file hasn't been written).
#
mtime() {
  stat -c "%Y" "$1"
}

chksum() {
  md5sum "$1"
}
