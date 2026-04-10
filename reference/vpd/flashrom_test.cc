// Copyright 2023 The ChromiumOS Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#include <stdint.h>

#include <optional>
#include <string>
#include <vector>

#include "base/files/file_path.h"
#include "base/files/scoped_temp_dir.h"
#include "gtest/gtest.h"

#include "vpd/fake_fmap.h"
#include "vpd/flashrom.h"
#include "vpd/types.h"

namespace vpd {

class FlashromTest : public ::testing::Test {
 protected:
  void SetUp() override {
    ASSERT_TRUE(temp_dir_.CreateUniqueTempDir());
    flash_path_ = temp_dir_.GetPath().Append("flash.bin");
  }

  void FillFlashFile(bool empty = false) {
    FakeFmap::FillFlashFile(flash_path_, 16 * 1024 * 1024, empty);
  }

  std::string GetProgrammerParams() const {
    return FakeFmap::GetProgrammerParams(flash_path_, 16 * 1024 * 1024);
  }

  base::ScopedTempDir temp_dir_;
  base::FilePath flash_path_;
};

TEST_F(FlashromTest, WriteReadBack) {
  FillFlashFile();

  Flashrom flashrom("dummy", GetProgrammerParams());

  std::vector<uint8_t> ro_in({0, 1, 2, 3});
  std::vector<uint8_t> rw_in({0xd, 0xe, 0xe, 0xa, 0xd});
  EXPECT_TRUE(flashrom.Write(VpdRw, rw_in));
  EXPECT_TRUE(flashrom.Write(VpdRo, ro_in));

  auto ro = flashrom.Read(VpdRo);
  ASSERT_NE(std::nullopt, ro);
  {
    unsigned int idx = 0;
    for (auto in : ro_in) {
      EXPECT_EQ(in, (*ro)[idx]);
      idx++;
    }
  }

  auto rw = flashrom.Read(VpdRw);
  ASSERT_NE(std::nullopt, rw);
  {
    unsigned int idx = 0;
    for (auto in : rw_in) {
      EXPECT_EQ(in, (*rw)[idx]);
      idx++;
    }
  }
}

TEST_F(FlashromTest, ReadEmptyRegions) {
  FillFlashFile();

  Flashrom flashrom("dummy", GetProgrammerParams());
  auto ro = flashrom.Read(VpdRo);
  ASSERT_NE(std::nullopt, ro);
  auto rw = flashrom.Read(VpdRw);
  ASSERT_NE(std::nullopt, rw);

  // NB: SetUp() filled our empty file with zeroes.
  for (auto i : *ro) {
    EXPECT_EQ(0, i);
  }
  for (auto i : *rw) {
    EXPECT_EQ(0, i);
  }
}

TEST_F(FlashromTest, EmptyFmap) {
  FillFlashFile(true /* empty */);

  Flashrom flashrom("dummy", GetProgrammerParams());

  EXPECT_FALSE(flashrom.Write(VpdRo, std::vector<uint8_t>({1, 2, 3})));
  EXPECT_FALSE(flashrom.Write(VpdRw, std::vector<uint8_t>({1, 2, 3})));

  EXPECT_EQ(std::nullopt, flashrom.Read(VpdRo));
  EXPECT_EQ(std::nullopt, flashrom.Read(VpdRw));
}

}  // namespace vpd
