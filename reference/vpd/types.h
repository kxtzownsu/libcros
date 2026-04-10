// Copyright 2023 The ChromiumOS Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef TYPES_H_
#define TYPES_H_

#include <string>

namespace vpd {

enum VpdRegion {
  VpdRo,
  VpdRw,
};

struct KeyVal {
  std::string key;
  std::string value;
  bool operator==(const KeyVal& other) const {
    return key == other.key && value == other.value;
  }
};

}  // namespace vpd

#endif  // TYPES_H_
