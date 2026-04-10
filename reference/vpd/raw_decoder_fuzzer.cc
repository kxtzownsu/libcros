// Copyright 2023 The ChromiumOS Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#include <stddef.h>
#include <stdint.h>

#include <vector>

#include "base/logging.h"

#include "vpd/encoder.h"

namespace vpd {

extern "C" int LLVMFuzzerTestOneInput(const uint8_t* data, size_t size) {
  // Turn off logging.
  logging::SetMinLogLevel(logging::LOGGING_FATAL);

  Encoder::DecodeRaw(std::vector<uint8_t>(data, data + size));

  return 0;
}

}  // namespace vpd
