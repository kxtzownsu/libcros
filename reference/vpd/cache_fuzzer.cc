// Copyright 2023 The ChromiumOS Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#include <stddef.h>
#include <stdint.h>

#include <vector>

#include "base/check.h"
#include "base/files/file_path.h"
#include "base/files/scoped_temp_dir.h"
#include "base/logging.h"

#include "vpd/cache.h"
#include "vpd/fake_fmap.h"
#include "vpd/flashrom.h"
#include "vpd/sysfs.h"
#include "vpd/types.h"

namespace vpd {

extern "C" int LLVMFuzzerTestOneInput(const uint8_t* data, size_t size) {
  // Turn off logging.
  logging::SetMinLogLevel(logging::LOGGING_FATAL);

  base::ScopedTempDir temp_dir;
  CHECK(temp_dir.CreateUniqueTempDir());

  unsigned int flash_size = 16 * 1024 * 1024;
  base::FilePath flash_path(temp_dir.GetPath().Append("flash.bin"));

  // Create a file-backed "flash", with an FMAP.
  FakeFmap::FillFlashFile(flash_path, flash_size, false /* empty fmap */);

  // Fill the RO_VPD region of that "flash" with fuzzed data.
  Flashrom flashrom("dummy",
                    FakeFmap::GetProgrammerParams(flash_path, flash_size));
  flashrom.Write(VpdRo, std::vector<uint8_t>(data, data + size));

  // Fake sysfs.
  Sysfs sysfs(temp_dir.GetPath().value());

  // Fire away!
  Cache cache(flash_path.value(), flash_size, temp_dir.GetPath().value(),
              sysfs);
  auto dict = cache.GetValues(VpdRo);

  // A valid dict is all we're looking for.

  return 0;
}

}  // namespace vpd
