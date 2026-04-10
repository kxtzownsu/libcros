// Copyright 2023 The ChromiumOS Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef FAKE_VPD_H_
#define FAKE_VPD_H_

#include <map>
#include <optional>
#include <string>

#include "vpd/export.h"
#include "vpd/types.h"
#include "vpd/vpd_provider_interface.h"

namespace vpd {

// A fake VPD implementation, for tests.
class EXPORT FakeVpd : public VpdProviderInterface {
 public:
  FakeVpd() {}

  ~FakeVpd() override {}

  bool Valid(VpdRegion region) const override { return true; }

  std::optional<std::string> GetValue(VpdRegion region,
                                      const std::string& key) const override {
    const auto& map = store_.at(region);
    const auto& it = map.find(key);
    if (it == map.end()) {
      return {};
    }

    return it->second;
  }

  std::map<std::string, std::string> GetValues(
      VpdRegion region) const override {
    return store_.at(region);
  }

  bool WriteValues(
      VpdRegion region,
      const std::map<std::string, std::optional<std::string>>& pairs) override {
    for (const auto& pair : pairs) {
      if (pair.second) {
        store_[region][pair.first] = *pair.second;
      } else {
        store_[region].erase(pair.first);
      }
    }

    return true;
  }

 private:
  std::map<VpdRegion, std::map<std::string, std::string>> store_ = {
      {VpdRo, {}},
      {VpdRw, {}},
  };
};

}  // namespace vpd

#endif  // FAKE_VPD_H_
