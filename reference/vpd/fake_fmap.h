// Copyright 2023 The ChromiumOS Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef FAKE_FMAP_H_
#define FAKE_FMAP_H_

#include <string>

#include "base/files/file_path.h"
#include "base/strings/stringprintf.h"

namespace vpd {

class FakeFmap {
 public:
  static void FillFlashFile(const base::FilePath& flash_path,
                            unsigned int size,
                            bool empty_fmap = false);

  static std::string GetProgrammerParams(const base::FilePath& flash_path,
                                         unsigned int size) {
    return base::StringPrintf("emulate=VARIABLE_SIZE,size=%u,image=%s", size,
                              flash_path.value().c_str());
  }
};

}  // namespace vpd

#endif  // FAKE_FMAP_H_
