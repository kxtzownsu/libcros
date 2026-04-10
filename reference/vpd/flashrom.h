// Copyright 2023 The ChromiumOS Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.
//
// NOTE: this libflashrom wrapper is not recommended for wide use. ChromeOS
// firmware updaters intend to funnel flash programming through futility
// eventually.
//
// NOTE: we currently rely on (lib)flashrom to acquire a system lock to broker
// concurrent flash readers/writers. This is not an upstream flashrom feature,
// and shouldn't be relied on forever. However, an alternative doesn't yet
// exist.

#ifndef FLASHROM_H_
#define FLASHROM_H_

extern "C" {
#include <libflashrom.h>
}  // extern "C"
#include <stdint.h>

#include <optional>
#include <string>
#include <vector>

#include "vpd/export.h"
#include "vpd/types.h"

namespace vpd {

class EXPORT Flashrom {
 public:
  Flashrom() : Flashrom("internal", "") {}
  Flashrom(std::string programmer, std::string params)
      : programmer_name_(programmer), programmer_params_(params) {}
  ~Flashrom();
  bool GetPartitionDimensions(VpdRegion region,
                              uint32_t* offset,
                              uint32_t* len);
  bool Write(VpdRegion region, const std::vector<uint8_t>& blob);
  std::optional<std::vector<uint8_t>> Read(VpdRegion region);

 private:
  bool Init();

  bool initialized_ = false;
  std::string programmer_name_;
  std::string programmer_params_;
  struct flashrom_programmer* programmer_ = nullptr;
  struct flashrom_flashctx* flashctx_ = nullptr;
  struct flashrom_layout* layout_ = nullptr;
};

}  // namespace vpd

#endif  // FLASHROM_H_
