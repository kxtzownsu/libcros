// Copyright 2023 The ChromiumOS Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#include "vpd/cache.h"

#include <stdint.h>

#include <map>
#include <optional>
#include <string>
#include <vector>

#include "absl/log/check.h"
#include "absl/log/log.h"
#include "absl/strings/str_format.h"

#include "vpd/cache_file.h"
#include "vpd/encoder.h"
#include "vpd/flashrom.h"
#include "vpd/types.h"

namespace vpd {

namespace {

std::vector<KeyVal> MapToVector(const std::map<std::string, std::string>& map) {
  std::vector<KeyVal> res;
  res.reserve(map.size());
  for (const auto& kv : map) {
    res.push_back(KeyVal{
        .key = kv.first,
        .value = kv.second,
    });
  }
  return res;
}

}  // namespace

Cache::Cache(const std::string& fake_flash_path,
             unsigned int size,
             const std::string& cache_dir,
             const Sysfs& sysfs)
    : programmer_args_(absl::StrFormat("emulate=VARIABLE_SIZE,size=%u,image=%s",
                                       size,
                                       fake_flash_path.c_str())),
      sysfs_(sysfs),
      cache_file_({
          {VpdRo, CacheFile(VpdRo, cache_dir)},
          {VpdRw, CacheFile(VpdRw, cache_dir)},
      }) {}

Cache::~Cache() {}

bool Cache::Valid(VpdRegion region) const {
  // If a cache file exists, that means we (over)wrote it => valid.
  // If the cache file doesn't exist but the sysfs entry does, then we had a
  // valid VPD at boot, and we trust it.
  // Otherwise, we have no valid VPD => invalid.
  return cache_file_.at(region).Exists() || sysfs_.Exists(region);
}

// Pull key/value map from either the cache file or from sysfs.
std::map<std::string, std::string> Cache::ReadValues(VpdRegion region) const {
  std::vector<KeyVal> pairs;

  if (cache_file_.at(region).Exists()) {
    std::optional<std::vector<KeyVal>> r = cache_file_.at(region).Read();
    CHECK(r.has_value());
    pairs.swap(r.value());
  } else {
    pairs = sysfs_.GetValues(region);
  }

  std::map<std::string, std::string> ret;
  for (const auto& kv : pairs) {
    ret[kv.key] = kv.value;
  }

  return ret;
}

bool Cache::WriteBack(VpdRegion region,
                      const std::map<std::string, std::string>& kvs) {
  auto flashrom = GetFlashrom();

  uint32_t partition_offset, partition_len;
  if (!flashrom.GetPartitionDimensions(region, &partition_offset,
                                       &partition_len)) {
    LOG(ERROR) << "Failed to determine partition dimensions";
    return false;
  }

  const auto& blob = Encoder::Encode(
      Encoder::EncodingParams{
          .partition_offset = partition_offset,
      },
      kvs);
  if (!blob) {
    LOG(ERROR) << "failed to encode VPD blob";
    return false;
  }

  // Write the flash first, in case there are obvious errors (e.g., no flash
  // device). We write the cache file afterward, once we're sure we succeed.
  if (!flashrom.Write(region, *blob)) {
    LOG(ERROR) << "flashrom failure";
    return false;
  }

  if (!cache_file_.at(region).Write(MapToVector(kvs))) {
    LOG(ERROR) << "Failed to write cache";
    return false;
  }

  return true;
}

Flashrom Cache::GetFlashrom() const {
  if (programmer_args_) {
    return Flashrom("dummy", *programmer_args_);
  }

  return Flashrom();
}

std::optional<std::string> Cache::GetValue(VpdRegion region,
                                           const std::string& key) const {
  const auto map = ReadValues(region);
  const auto& it = map.find(key);
  if (it != map.end()) {
    return it->second;
  }
  return {};
}

std::map<std::string, std::string> Cache::GetValues(VpdRegion region) const {
  return ReadValues(region);
}

bool Cache::WriteValues(
    VpdRegion region,
    const std::map<std::string, std::optional<std::string>>& pairs) {
  if (pairs.empty()) {
    LOG(ERROR) << "nothing to write";
    return false;
  }

  auto map = ReadValues(region);

  for (const auto& pair : pairs) {
    if (pair.second) {
      map[pair.first] = *pair.second;
    } else {
      map.erase(pair.first);
    }
  }

  return WriteBack(region, map);
}

}  // namespace vpd
