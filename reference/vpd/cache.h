// Copyright 2023 The ChromiumOS Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef CACHE_H_
#define CACHE_H_

#include <map>
#include <optional>
#include <string>

#include "vpd/cache_file.h"
#include "vpd/export.h"
#include "vpd/flashrom.h"
#include "vpd/sysfs.h"
#include "vpd/types.h"
#include "vpd/vpd_provider_interface.h"

namespace vpd {

// A VPD implementation that utilizes a tmpfs cache, and writes back to the
// flash.
class EXPORT Cache : public VpdProviderInterface {
 public:
  Cache() {}
  // Only for tests.
  Cache(const std::string& fake_flash_path,
        unsigned int size,
        const std::string& cache_dir,
        const Sysfs& sysfs);

  ~Cache() override;

  bool Valid(VpdRegion region) const override;

  std::optional<std::string> GetValue(VpdRegion region,
                                      const std::string& key) const override;

  std::map<std::string, std::string> GetValues(VpdRegion region) const override;

  bool WriteValues(
      VpdRegion region,
      const std::map<std::string, std::optional<std::string>>& pairs) override;

 private:
  std::map<std::string, std::string> ReadValues(VpdRegion region) const;
  bool WriteBack(VpdRegion region,
                 const std::map<std::string, std::string>& kvs);
  Flashrom GetFlashrom() const;

  // Flashrom programmer args; only for tests.
  std::optional<std::string> programmer_args_;

  Sysfs sysfs_;
  std::map<VpdRegion, CacheFile> cache_file_ = {
      {VpdRo, CacheFile(VpdRo)},
      {VpdRw, CacheFile(VpdRw)},
  };
};

}  // namespace vpd

#endif  // CACHE_H_
