// Copyright 2023 The ChromiumOS Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#include <array>
#include <optional>
#include <string>
#include <vector>

#include "base/files/file_path.h"
#include "base/files/file_util.h"
#include "base/files/scoped_temp_dir.h"
#include "gtest/gtest.h"

#include "vpd/cache_file.h"
#include "vpd/types.h"
#include "vpd/util/scoped_umask.h"

namespace vpd {

using std::string_literals::operator""s;

class CacheFileTest : public ::testing::Test {
 protected:
  void SetUp() override {
    ASSERT_TRUE(temp_dir_.CreateUniqueTempDir());
    base_dir_ = temp_dir_.GetPath();
  }

  base::ScopedTempDir temp_dir_;
  base::FilePath base_dir_;
};

TEST_F(CacheFileTest, Exists) {
  {
    CacheFile cache(VpdRo, base_dir_.value());

    ASSERT_FALSE(cache.Exists());
    std::string data = "";
    base::WriteFile(base_dir_.Append("ro.txt"), data);
    ASSERT_TRUE(cache.Exists());
  }
  {
    CacheFile cache(VpdRw, base_dir_.value());
    ASSERT_FALSE(cache.Exists());
    std::string data = "";
    base::WriteFile(base_dir_.Append("rw.txt"), data);
    ASSERT_TRUE(cache.Exists());
  }
}

TEST_F(CacheFileTest, Read) {
  struct expectation {
    std::string contents;
    std::vector<KeyVal> kvs;
    bool expect_success;
  };
  std::array<expectation, 5> expectations = {
      (struct expectation){
          .contents = "foo=\"bar\"\0"s,
          .kvs =
              {
                  {
                      .key = "foo",
                      .value = "bar",
                  },
              },
          .expect_success = true,
      },
      (struct expectation){
          .contents = "foo=\"bar\"\0this=\"has\"quotes\"\0that=\"="s
                      "extraequals\"\0empty=\"\"\0"s,
          .kvs =
              {
                  {
                      .key = "foo",
                      .value = "bar",
                  },
                  {
                      .key = "this",
                      .value = "has\"quotes",
                  },
                  {
                      .key = "that",
                      .value = "=extraequals",
                  },
                  {
                      .key = "empty",
                      .value = "",
                  },
              },
          .expect_success = true,
      },
      (struct expectation){
          .contents = "",
          .kvs = {},
          .expect_success = true,
      },
      (struct expectation){
          .contents = "malformatted",
          .expect_success = false,
      },
      (struct expectation){
          .contents = "notenough=quotes\"",
          .expect_success = false,
      },
  };

  for (const auto& expect : expectations) {
    CacheFile cache(VpdRo, base_dir_.value());

    base::WriteFile(base_dir_.Append("ro.txt"), expect.contents);
    std::optional<std::vector<KeyVal>> result = cache.Read();
    EXPECT_EQ(result.has_value(), expect.expect_success);
    if (expect.expect_success) {
      EXPECT_EQ(result.value(), expect.kvs);
    }
  }
}

TEST_F(CacheFileTest, Write) {
  struct expectation {
    std::vector<KeyVal> kvs;
    std::string contents;
  };
  std::array<expectation, 3> expectations = {
      (struct expectation){
          .kvs =
              {
                  {
                      .key = "foo",
                      .value = "bar",
                  },
              },
          .contents = "foo=\"bar\"\0"s,
      },
      (struct expectation){
          .kvs =
              {
                  {
                      .key = "foo",
                      .value = "bar",
                  },
                  {
                      .key = "this",
                      .value = "has\"quotes",
                  },
                  {
                      .key = "that",
                      .value = "=extraequals",
                  },
                  {
                      .key = "empty",
                      .value = "",
                  },
              },
          .contents = "foo=\"bar\"\0this=\"has\"quotes\"\0that=\"="s
                      "extraequals\"\0empty=\"\"\0"s,
      },
      (struct expectation){
          .kvs = {},
          .contents = "",
      },
  };

  for (const auto& expect : expectations) {
    CacheFile cache(VpdRo, base_dir_.value());

    EXPECT_TRUE(cache.Write(expect.kvs));
    std::string data;
    ASSERT_TRUE(base::ReadFileToString(base_dir_.Append("ro.txt"), &data));
    EXPECT_EQ(data, expect.contents);
  }
}

TEST_F(CacheFileTest, Bidirectional) {
  std::array<std::vector<KeyVal>, 3> kvs = {
      std::vector<KeyVal>(),
      {
          {
              .key = "foo",
              .value = "bar",
          },
      },
      {
          {
              .key = "extra",
              .value = "quote\"s",
          },
          {
              .key = "empty",
              .value = "",
          },
          {
              .key = "equals",
              .value = "=some=val",
          },
      },
  };

  for (const auto& kv : kvs) {
    CacheFile cache(VpdRw, base_dir_.value());
    EXPECT_TRUE(cache.Write(kv));
    std::optional<std::vector<KeyVal>> result = cache.Read();
    EXPECT_TRUE(result.has_value());
    EXPECT_EQ(kv, result);
  }
}

TEST_F(CacheFileTest, Umask) {
  vpd::util::ScopedUmask outer_mask(0066);
  int mode;

  // Ensure umask is taking effect.
  base::FilePath junk_path = base_dir_.Append("junk");
  EXPECT_TRUE(base::WriteFile(junk_path, "nothing to see here"));
  EXPECT_TRUE(base::GetPosixFilePermissions(junk_path, &mode));
  EXPECT_EQ(mode, 0600);

  // Now ensure cache file ignores the preexisting umask.
  CacheFile cache(VpdRw, base_dir_.value());
  EXPECT_TRUE(cache.Write({{
      .key = "foo",
      .value = "bar",
  }}));

  base::FilePath cache_path = base_dir_.Append("rw.txt");
  EXPECT_TRUE(base::GetPosixFilePermissions(cache_path, &mode));
  EXPECT_EQ(mode, 0644);
}

}  // namespace vpd
