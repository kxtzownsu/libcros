// Copyright 2023 The ChromiumOS Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// Helpers for util/update_rw_vpd.cc. Dumped in a header for ease of
// util/update_rw_vpd_test.cc.

#ifndef UTIL_UPDATE_RW_VPD_H_
#define UTIL_UPDATE_RW_VPD_H_

#include <map>
#include <optional>
#include <string>

#include "absl/log/log.h"

#include "vpd/vpd.h"

namespace update_rw_vpd {

std::optional<std::map<std::string, std::optional<std::string>>> ParseArgs(
    int argc, const char* argv[]) {
  std::map<std::string, std::optional<std::string>> write_values;

  for (int i = 1; i < argc; i++) {
    std::string key = argv[i];
    i++;
    if (i >= argc) {
      LOG(ERROR) << "Invalid args";
      return {};
    }
    std::string value = argv[i];

    if (key.find_first_not_of("abcdefghijklmnopqrstuvwxyz"
                              "ABCDEFGHIJKLMNOPQRSTUVWXYZ"
                              "0123456789_") != std::string::npos) {
      LOG(ERROR) << "Invalid key name: " << key;
      return {};
    }

    if (value.empty()) {
      // Empty args are treated as "delete" by convention.
      write_values[key] = std::nullopt;
    } else {
      write_values[key] = value;
    }
  }

  return write_values;
}

bool NeedsWrite(
    vpd::Vpd& vpd,
    const std::map<std::string, std::optional<std::string>> write_values) {
  const auto& kv = vpd.GetValues(vpd::VpdRw);

  for (const auto& pair : write_values) {
    const auto& it = kv.find(pair.first);

    if (pair.second) {
      // This is a write; check if the key/value already exists and matches.
      if (it == kv.end() || it->second != *pair.second) {
        return true;
      }
    } else {
      // This is a deletion; check if the key already exists.
      if (it != kv.end()) {
        return true;
      }
    }
  }

  return false;
}

}  // namespace update_rw_vpd

#endif  // UTIL_UPDATE_RW_VPD_H_
