// Copyright 2023 The ChromiumOS Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef VPD_H_
#define VPD_H_

#include <map>
#include <memory>
#include <optional>
#include <string>

#include "vpd/export.h"
#include "vpd/types.h"

namespace vpd {

class VpdProviderInterface;

class EXPORT Vpd {
 public:
  Vpd();
  // For tests, to provide a fake.
  explicit Vpd(std::unique_ptr<VpdProviderInterface> provider);
  ~Vpd();

  // Check if a VPD region has valid contents.
  //
  // This operation checks if a VPD region has valid contents. A region is
  // considered invalid if it doesn't have appropriate formatting and headers
  // -- for example, if the flash region is corrupt or erased and couldn't be
  // parsed. A region may become valid again if a subsequent write operation
  // programs new contents.
  //
  // @param region The region to check.
  bool Valid(VpdRegion region) const;

  // Read a single key VPD value from a VPD region.
  //
  // @param region The VPD region to read from.
  // @param key The VPD key to locate.
  // @return The value if it exists, or |std::nullopt| otherwise.
  std::optional<std::string> GetValue(VpdRegion region,
                                      const std::string& key) const;

  // Read all key/value pairs from a VPD region.
  //
  // @param region The VPD region to read from.
  // @return A map of key/value pairs.
  std::map<std::string, std::string> GetValues(VpdRegion region);

  // Write or delete one or more key/value pairs to a VPD region.
  //
  // This operation can batch several writes (e.g., Vpd::WriteValue(),
  // Vpd::DeleteKey()). All operations are complete (or failed) by the time
  // this function returns.
  //
  // Note that it is not an error to delete a key that doesn't exist, nor to
  // write a key that already exists.
  //
  // @param region The VPD region to write to.
  // @param pairs A map of key/value pairs to write or delete. If a value is
  //   provided, then the key/value pair is written; if a value is empty
  //   (|std::nullopt|), then it is deleted.
  //   Must not be an empty map.
  // @return |true| if it succeeds; |false| on error.
  bool WriteValues(
      VpdRegion region,
      const std::map<std::string, std::optional<std::string>>& pairs);

  // Write a single key/value pair to a VPD region.
  //
  // Note that it is not an error to write a key that already exists.
  //
  // @param region The VPD region to write to.
  // @param key The key to write.
  // @param val The value to write.
  // @return |true| if it succeeds; |false| on error.
  bool WriteValue(VpdRegion region,
                  const std::string& key,
                  const std::string& val);

  // Delete a single key/value pair to a VPD region.
  //
  // Note that it is not an error to delete a key that doesn't exist.
  //
  // @param region The VPD region to write to.
  // @param key The key to write.
  // @param val The value to write.
  // @return |true| if it succeeds; |false| on error.
  bool DeleteKey(VpdRegion region, const std::string& key);

 private:
  std::unique_ptr<VpdProviderInterface> provider_;
};

}  // namespace vpd

#endif  // VPD_H_
