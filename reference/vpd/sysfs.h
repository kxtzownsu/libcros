// Copyright 2023 The ChromiumOS Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef SYSFS_H_
#define SYSFS_H_

#include <optional>
#include <string>
#include <vector>

#include "vpd/export.h"
#include "vpd/types.h"

namespace vpd {

class EXPORT Sysfs {
 public:
  Sysfs();
  explicit Sysfs(const std::string& sysfs_dir);

  // Check if the VPD region exists in sysfs.
  //
  // A VPD region will not exist in sysfs if the coreboot parser couldn't
  // understand it -- for example, if the region was erased or corrupt.
  //
  // @param region The VPD region to check.
  bool Exists(VpdRegion region) const;

  std::optional<std::string> GetValue(VpdRegion region,
                                      const std::string& key) const;

  std::vector<KeyVal> GetValues(VpdRegion region) const;

 private:
  std::string sysfs_dir_;
};

}  // namespace vpd

#endif  // SYSFS_H_
