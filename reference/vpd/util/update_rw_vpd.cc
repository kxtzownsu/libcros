// Copyright 2023 The ChromiumOS Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// Helper to update values (e.g., block_devmode) in VPD using cached results to
// avoid unnecessary flash reads and writes.
//
// WARNING: This tool must not be run with untrusted input.
//
// Call with params
//   key1 value1 key2 value2...
//
// The values can be arbitrary strings. Empty strings as values cause a
// deletion of the corresponding key. The keys can contain only alphanumeric
// ASCII characters and underscores.
//
// Example usage: update_rw_vpd block_devmode 1 check_enrollment 0

#include <iostream>

#include "absl/log/initialize.h"
#include "absl/log/log.h"

#include "vpd/util/update_rw_vpd.h"
#include "vpd/vpd.h"

int main(int argc, const char* argv[]) {
  absl::InitializeLog();

  const auto write_values = update_rw_vpd::ParseArgs(argc, argv);
  if (!write_values) {
    return 1;
  }

  vpd::Vpd vpd;

  if (!update_rw_vpd::NeedsWrite(vpd, *write_values)) {
    return 0;
  }

  for (const auto& pair : *write_values) {
    if (pair.second) {
      std::cout << "Update key " << pair.first << "=" << *pair.second
                << " in VPD\n";
    } else {
      std::cout << "Key " << pair.first << " to be removed from VPD\n";
    }
  }

  if (!vpd.WriteValues(vpd::VpdRw, *write_values)) {
    LOG(ERROR) << "Failed to write to VPD";
    return 1;
  }

  return 0;
}
