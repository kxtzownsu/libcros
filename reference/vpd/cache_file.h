// Copyright 2023 The ChromiumOS Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef CACHE_FILE_H_
#define CACHE_FILE_H_

#include <optional>
#include <string>
#include <vector>

#include "vpd/export.h"
#include "vpd/types.h"

namespace vpd {

class EXPORT CacheFile {
 public:
  explicit CacheFile(VpdRegion region);
  CacheFile(VpdRegion region, const std::string& cacheDir);
  bool Exists() const;
  std::optional<std::vector<KeyVal>> Read() const;
  bool Write(const std::vector<KeyVal>& kv) const;

 private:
  std::string path_;
};

}  // namespace vpd

#endif  // CACHE_FILE_H_
