// Copyright 2024 The ChromiumOS Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef UTIL_FILE_H_
#define UTIL_FILE_H_

#include <stdint.h>

#include <optional>
#include <string>
#include <vector>

#include "vpd/export.h"

namespace vpd {
namespace util {

EXPORT std::optional<std::vector<uint8_t>> ReadFileToBytes(
    const std::string& filename);

EXPORT std::optional<std::string> ReadFileToString(const std::string& filename);

EXPORT bool WriteFile(const std::string& filename, const std::string& data);

EXPORT std::vector<std::string> ListFiles(const std::string& dir);

EXPORT bool PathExists(const std::string& path);

EXPORT std::string JoinPath(const std::string& dir,
                            const std::string& basename);

}  // namespace util
}  // namespace vpd

#endif  // UTIL_FILE_H_
