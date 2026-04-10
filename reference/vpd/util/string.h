// Copyright 2024 The ChromiumOS Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef UTIL_STRING_H_
#define UTIL_STRING_H_

#include <stdarg.h>

#include <string>
#include <utility>
#include <vector>

#include "vpd/export.h"

namespace vpd {
namespace util {

// Parse key=value pairs. Returns true if the parsing succeeds.
EXPORT bool GetNullDelimitedKeyValuePairs(
    const std::string& content,
    std::vector<std::pair<std::string, std::string>>& result);

EXPORT std::string StringPrintV(const char* format, va_list args);

}  // namespace util
}  // namespace vpd

#endif  // UTIL_STRING_H_
