// Copyright 2023 The ChromiumOS Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#include <stddef.h>
#include <stdint.h>

#include <optional>
#include <vector>

#include "base/check.h"
#include "base/files/file_path.h"
#include "base/files/file_util.h"
#include "base/files/scoped_temp_dir.h"
#include "base/logging.h"

#include "vpd/cache_file.h"
#include "vpd/types.h"

namespace vpd {

extern "C" int LLVMFuzzerTestOneInput(const uint8_t* data, size_t size) {
  // Turn off logging.
  logging::SetMinLogLevel(logging::LOGGING_FATAL);

  base::ScopedTempDir temp_dir;
  CHECK(temp_dir.CreateUniqueTempDir());
  base::FilePath cache_file_path(temp_dir.GetPath().Append("ro.txt"));
  CHECK(base::WriteFile(cache_file_path,
                        std::vector<uint8_t>(data, data + size)));

  CacheFile cache_file(VpdRo, temp_dir.GetPath().value());
  std::optional<std::vector<KeyVal>> kv = cache_file.Read();

  return 0;
}

}  // namespace vpd
