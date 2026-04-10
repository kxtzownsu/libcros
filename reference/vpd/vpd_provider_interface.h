// Copyright 2023 The ChromiumOS Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef VPD_PROVIDER_INTERFACE_H_
#define VPD_PROVIDER_INTERFACE_H_

#include <map>
#include <optional>
#include <string>

#include "vpd/types.h"

namespace vpd {

// Class interface for the underlying VPD-providing implementation.
class VpdProviderInterface {
 public:
  virtual ~VpdProviderInterface() = default;

  virtual bool Valid(VpdRegion region) const = 0;

  // Read a single key VPD value from a VPD region.
  //
  // See vpd.h for API definition.
  virtual std::optional<std::string> GetValue(VpdRegion region,
                                              const std::string& key) const = 0;

  // Read all key/value pairs from a VPD region.
  //
  // See vpd.h for API definition.
  virtual std::map<std::string, std::string> GetValues(
      VpdRegion region) const = 0;

  // Write or delete one or more key/value pairs to a VPD region.
  //
  // See vpd.h for API definition.
  virtual bool WriteValues(
      VpdRegion region,
      const std::map<std::string, std::optional<std::string>>& pairs) = 0;
};

}  // namespace vpd

#endif  // VPD_PROVIDER_INTERFACE_H_
