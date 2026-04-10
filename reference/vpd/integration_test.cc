// Copyright 2023 The ChromiumOS Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#include <optional>
#include <string>

#include "base/environment.h"
#include "base/files/file_path.h"
#include "base/strings/stringprintf.h"
#include "gtest/gtest.h"

#include "vpd/encoder.h"
#include "vpd/flashrom.h"
#include "vpd/types.h"

namespace vpd {

// Integration of flashrom + encoder.
class IntegrationTest : public ::testing::Test {
 protected:
  base::FilePath GetBuildDir() {
    std::optional<std::string> out_dir =
        base::Environment::Create()->GetVar("OUT");
    EXPECT_TRUE(out_dir.has_value());
    return base::FilePath(*out_dir);
  }
};

// Malformed RO_VPD should fail to decode.
TEST_F(IntegrationTest, Broken) {
  auto file_path = GetBuildDir().Append("broken.vpd");
  // Emulate an 8MiB flash, to match the 8MiB *.vpd files.
  Flashrom flashrom(
      "dummy", base::StringPrintf("emulate=VARIABLE_SIZE,size=%u,image=%s",
                                  8 * 1024 * 1024, file_path.value().c_str()));
  auto blob = flashrom.Read(VpdRo);
  ASSERT_NE(std::nullopt, blob);

  Encoder::DecodedParams params;
  auto dict = Encoder::Decode(*blob, &params);
  EXPECT_EQ(std::nullopt, dict);
}

}  // namespace vpd
