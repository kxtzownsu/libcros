// Copyright 2023 The ChromiumOS Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#include "vpd/vpd.h"

#include <map>
#include <memory>
#include <optional>
#include <utility>
#include <string>

#include "vpd/cache.h"
#include "vpd/types.h"
#include "vpd/vpd_provider_interface.h"

namespace vpd {

Vpd::Vpd() : provider_(std::make_unique<Cache>()) {}

Vpd::Vpd(std::unique_ptr<VpdProviderInterface> provider)
    : provider_(std::move(provider)) {}

Vpd::~Vpd() = default;

bool Vpd::Valid(VpdRegion region) const {
  return provider_->Valid(region);
}

std::optional<std::string> Vpd::GetValue(VpdRegion region,
                                         const std::string& key) const {
  return provider_->GetValue(region, key);
}

std::map<std::string, std::string> Vpd::GetValues(VpdRegion region) {
  return provider_->GetValues(region);
}

bool Vpd::WriteValues(
    VpdRegion region,
    const std::map<std::string, std::optional<std::string>>& pairs) {
  return provider_->WriteValues(region, pairs);
}

bool Vpd::WriteValue(VpdRegion region,
                     const std::string& key,
                     const std::string& val) {
  return WriteValues(region, {{key, val}});
}

bool Vpd::DeleteKey(VpdRegion region, const std::string& key) {
  return WriteValues(region, {{key, std::nullopt}});
}

}  // namespace vpd
