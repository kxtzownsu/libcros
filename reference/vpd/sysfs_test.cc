// Copyright 2023 The ChromiumOS Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#include <algorithm>
#include <optional>

#include "base/files/file_path.h"
#include "base/files/file_util.h"
#include "base/files/scoped_temp_dir.h"
#include "gtest/gtest.h"

#include "vpd/sysfs.h"
#include "vpd/types.h"

namespace vpd {

class SysfsTest : public ::testing::Test {
 protected:
  void SetUp() override {
    ASSERT_TRUE(temp_dir_.CreateUniqueTempDir());
    sysfs_dir_ = temp_dir_.GetPath();
  }

  base::ScopedTempDir temp_dir_;
  base::FilePath sysfs_dir_;
};

TEST_F(SysfsTest, GetValue) {
  ASSERT_TRUE(base::CreateDirectory(sysfs_dir_.Append("ro")));
  ASSERT_TRUE(base::CreateDirectory(sysfs_dir_.Append("rw")));
  ASSERT_TRUE(base::WriteFile(sysfs_dir_.Append("ro").Append("foo"), "bar"));
  ASSERT_TRUE(base::WriteFile(sysfs_dir_.Append("rw").Append("baz"), ""));

  Sysfs sysfs(sysfs_dir_.value());

  EXPECT_EQ(*sysfs.GetValue(VpdRo, "foo"), "bar");
  EXPECT_EQ(*sysfs.GetValue(VpdRw, "baz"), "");
  EXPECT_EQ(sysfs.GetValue(VpdRw, "nonexist"), std::nullopt);
}

TEST_F(SysfsTest, GetValues) {
  ASSERT_TRUE(base::CreateDirectory(sysfs_dir_.Append("ro")));
  ASSERT_TRUE(base::CreateDirectory(sysfs_dir_.Append("rw")));
  ASSERT_TRUE(base::WriteFile(sysfs_dir_.Append("ro").Append("foo"), "bar"));
  ASSERT_TRUE(
      base::WriteFile(sysfs_dir_.Append("ro").Append("another"), "entry"));
  ASSERT_TRUE(base::WriteFile(sysfs_dir_.Append("rw").Append("baz"), ""));

  Sysfs sysfs(sysfs_dir_.value());

  auto ro = sysfs.GetValues(VpdRo);
  auto rw = sysfs.GetValues(VpdRw);

  EXPECT_EQ(ro.size(), 2);
  EXPECT_NE(std::find(ro.begin(), ro.end(),
                      (KeyVal){
                          .key = "foo",
                          .value = "bar",
                      }),
            ro.end());
  EXPECT_NE(std::find(ro.begin(), ro.end(),
                      (KeyVal){
                          .key = "another",
                          .value = "entry",
                      }),
            ro.end());

  EXPECT_EQ(rw.size(), 1);
  EXPECT_EQ(rw[0], ((KeyVal){
                       .key = "baz",
                       .value = "",
                   }));
}

TEST_F(SysfsTest, Exist) {
  Sysfs sysfs(sysfs_dir_.value());

  // Neither ro nor rw exist.
  ASSERT_FALSE(base::DirectoryExists(sysfs_dir_.Append("ro")));
  ASSERT_FALSE(base::DirectoryExists(sysfs_dir_.Append("rw")));
  EXPECT_FALSE(sysfs.Exists(VpdRo));
  EXPECT_FALSE(sysfs.Exists(VpdRw));

  // Just ro exists.
  ASSERT_TRUE(base::CreateDirectory(sysfs_dir_.Append("ro")));
  EXPECT_TRUE(sysfs.Exists(VpdRo));
  EXPECT_FALSE(sysfs.Exists(VpdRw));

  // Both ro and rw exist.
  ASSERT_TRUE(base::CreateDirectory(sysfs_dir_.Append("rw")));
  EXPECT_TRUE(sysfs.Exists(VpdRo));
  EXPECT_TRUE(sysfs.Exists(VpdRw));

  // Just rw exists.
  ASSERT_TRUE(base::DeleteFile(sysfs_dir_.Append("ro")));
  EXPECT_FALSE(sysfs.Exists(VpdRo));
  EXPECT_TRUE(sysfs.Exists(VpdRw));
}

}  // namespace vpd
