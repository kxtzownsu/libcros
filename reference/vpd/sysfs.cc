// Copyright 2023 The ChromiumOS Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#include "vpd/sysfs.h"

#include <optional>
#include <string>
#include <vector>

#include "absl/log/check.h"
#include "absl/log/log.h"

#include "vpd/types.h"
#include "vpd/util/file.h"

namespace vpd {

namespace {

constexpr char kSysfsVpd[] = "/sys/firmware/vpd";

std::string VpdRegionToSysfsPath(std::string sysfs_dir, VpdRegion region) {
  if (region == VpdRo) {
    return vpd::util::JoinPath(sysfs_dir, "ro");
  }
  if (region == VpdRw) {
    return vpd::util::JoinPath(sysfs_dir, "rw");
  }
  __builtin_unreachable();
}

std::optional<std::string> VpdSysfsRead(const std::string& region_dir,
                                        const std::string& key) {
  auto path = vpd::util::JoinPath(region_dir, key);

  auto contents = vpd::util::ReadFileToString(path);
  if (!contents) {
    return {};
  }

  return contents;
}

}  // namespace

using std::string_literals::operator""s;

Sysfs::Sysfs() : Sysfs(kSysfsVpd) {}

Sysfs::Sysfs(const std::string& sysfs_dir) : sysfs_dir_(sysfs_dir) {}

bool Sysfs::Exists(VpdRegion region) const {
  return vpd::util::PathExists(VpdRegionToSysfsPath(sysfs_dir_, region));
}

std::optional<std::string> Sysfs::GetValue(VpdRegion region,
                                           const std::string& key) const {
  return VpdSysfsRead(VpdRegionToSysfsPath(sysfs_dir_, region), key);
}

std::vector<KeyVal> Sysfs::GetValues(VpdRegion region) const {
  auto dir = VpdRegionToSysfsPath(sysfs_dir_, region);
  std::vector<KeyVal> res;

  for (const std::string& key : vpd::util::ListFiles(dir)) {
    auto contents = vpd::util::ReadFileToString(vpd::util::JoinPath(dir, key));
    if (!contents) {
      PLOG(ERROR) << "Failed to read sysfs: " << key;
      continue;
    }
    if (contents.value().find_first_of("\0"s) != std::string::npos) {
      continue;
    }
    res.push_back(KeyVal{
        .key = key,
        .value = contents.value(),
    });
  }

  return res;
}

}  // namespace vpd
