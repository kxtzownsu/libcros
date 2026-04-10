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

  Encoder::DecodedParams params;
  auto dict = Encoder::Decode(std::vector<uint8_t>(data, data + size), &params);

  // If we decoded something, why not try re-encoding it?
  if (dict) {
    Encoder::EncodingParams encoding_params = {
        .partition_offset = 0,
        .eps_offset = params.eps_offset,
    };
    // Don't assume this will succeed, as we might be a bit stricter on
    // encoding inputs. Just fuzz that we don't do Real Bad Things.
    Encoder::Encode(encoding_params, *dict);
  }

  return 0;
}

}  // namespace vpd
