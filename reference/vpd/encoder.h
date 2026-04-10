// Copyright 2023 The ChromiumOS Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef ENCODER_H_
#define ENCODER_H_

#include <stdint.h>

#include <map>
#include <optional>
#include <string>
#include <vector>

#include "vpd/export.h"

namespace vpd {

class EXPORT Encoder {
 public:
  struct EncodingParams {
    // Offset of the VPD region within the flash.
    uint32_t partition_offset = 0;
    // All offsets are offsets within the flash partition.
    uint32_t eps_offset = 0;
  };

  static std::optional<std::vector<uint8_t>> Encode(
      const EncodingParams& params,
      const std::map<std::string, std::string>& dict);

  struct DecodedParams {
    // All offsets are offsets within the flash partition.
    uint32_t eps_offset = 0;
  };

  static std::optional<std::map<std::string, std::string>> Decode(
      const std::vector<uint8_t>& blob, DecodedParams* params);
  static std::optional<std::map<std::string, std::string>> DecodeRaw(
      const std::vector<uint8_t>& blob);
};

}  // namespace vpd

#endif  // ENCODER_H_
