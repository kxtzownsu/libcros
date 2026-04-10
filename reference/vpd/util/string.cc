// Copyright 2024 The ChromiumOS Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#include "vpd/util/string.h"

#include <stdarg.h>
#include <stddef.h>
#include <stdio.h>

#include <string>
#include <utility>
#include <vector>

#include "absl/log/log.h"
#include "absl/strings/str_split.h"

namespace {

// Return true if memory size is sufficient and vsnprintf succeeds.
bool TryStringPrintV(size_t mem_length,
                     std::string& dst,
                     const char* format,
                     va_list ap) {
  std::vector<char> mem_buf(mem_length);
  va_list ap_copy;
  va_copy(ap_copy, ap);
#pragma GCC diagnostic ignored "-Wformat-nonliteral"
  int result = vsnprintf(mem_buf.data(), mem_length, format, ap_copy);
#pragma GCC diagnostic warning "-Wformat-nonliteral"
  va_end(ap_copy);

  if (result >= 0 && (static_cast<size_t>(result) < mem_length)) {
    dst.append(mem_buf.data(), static_cast<size_t>(result));
    return true;
  }

  return false;
}

}  // namespace

namespace vpd {
namespace util {

bool GetNullDelimitedKeyValuePairs(
    const std::string& content,
    std::vector<std::pair<std::string, std::string>>& result) {
  std::vector<std::string> entries = absl::StrSplit(content, '\0');

  // Remove trailing separator.
  if (!entries.empty() && entries.back().empty()) {
    entries.pop_back();
  }

  for (auto entry : entries) {
    std::pair<std::string, std::string> kv =
        absl::StrSplit(entry, absl::MaxSplits('=', 1));
    if (kv.first.empty() || kv.second.empty()) {
      return false;
    }
    result.push_back(kv);
  }
  return true;
}

std::string StringPrintV(const char* format, va_list args) {
  std::string result;
  // Borrow the size limit from libchrome design.
  for (size_t size = 1024; size <= 32 * 1024 * 1024; size *= 2) {
    if (TryStringPrintV(size, result, format, args)) {
      return result;
    }
  }

  DLOG(WARNING) << "Unable to printf the requested string due to size.";
  return "";
}

}  // namespace util
}  // namespace vpd
